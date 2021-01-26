package ren.home.bingeAtHome.service.impl;

import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.io.UrlResource;
import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpRange;
import org.springframework.stereotype.Service;
import ren.home.bingeAtHome.dao.MetadataDao;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.model.VideoMetadata;
import ren.home.bingeAtHome.service.VideoService;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.io.File;
import java.io.IOException;
import java.nio.file.InvalidPathException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/**
 * Base implementation of VideoService.
 *
 * @author Attila Szőke
 */
@Slf4j
@Service
public class VideoServiceImpl implements VideoService {

    private final VideoDao videoDao;
    private final MetadataDao metadataDao;

    /**
     * Instantiates a new Video service.
     *
     * @param videoDao    the video dao
     * @param metadataDao the metadata dao
     */
    @Autowired
    public VideoServiceImpl(VideoDao videoDao, MetadataDao metadataDao) {
        this.videoDao = videoDao;
        this.metadataDao = metadataDao;
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public List<Video> getAllVideos() {
        List<Video> storedVideos = new ArrayList<>();
        for (File file : videoDao.findAllVideoFiles()) {
            try {
                storedVideos.add(new Video(file));
            } catch (IOException | InterruptedException e) {
                log.warn("Video fetched is now missing somehow: {}!", file.getAbsolutePath());
            }
        }
        Collections.sort(storedVideos);
        log.debug("Videos fetched: {}!", storedVideos);
        return storedVideos;
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public Video getVideo(String fileName) throws VideoMissingException {
        File videoFile;
        VideoMetadata videoMetadata = null;
        try {
            videoFile = videoDao.getVideoFile(fileName);
        } catch (IOException e) {
            log.debug("Video with this name does not exist: {}!", fileName);
            throw new VideoMissingException();
        }
        try {
            videoMetadata = metadataDao.readMetadata(fileName);
        } catch (IOException e) {
            log.warn("Metadata for this file is missing: {}!", fileName);
        }
        try {
            Video video = new Video(videoFile, videoMetadata);
            log.debug("Fetched video: {}", video);
            return video;
        } catch (IOException | InterruptedException e) {
            log.warn("Video fetched is now missing somehow: {}!", fileName);
            throw new VideoMissingException();
        }
    }

    /**
     * {@inheritDoc}
     *
     * @return
     */
    @Override
    public ResourceRegion prepareContent(String videoName, HttpHeaders headers) throws VideoMissingException {
        try {
            UrlResource resource = videoDao.findResourceByName(videoName);
            ResourceRegion region = resourceRegion(resource, headers);
            log.debug("Prepared video region {}, length: {}!", videoName, region.getCount());
            return region;
        } catch (IOException | InvalidPathException e) {
            log.warn("Video fetched is missing: {}!", videoName);
            throw new VideoMissingException();
        }
    }

    /**
     * Returns a new ResourceRegion object, which represents a range from the byte-array of the video.
     *
     * @param resource video resource
     * @param headers  headers of the request, among them the range header is relevant
     * @return ResourceRegion object
     * @throws IOException thrown when something is amiss with the video resource
     */
    private ResourceRegion resourceRegion(UrlResource resource, HttpHeaders headers) throws IOException {
        long contentLength = resource.contentLength();
        HttpRange range = headers.getRange().stream().findFirst().orElse(null);
        if (range != null) {
            long start = range.getRangeStart(contentLength);
            long end = range.getRangeEnd(contentLength);
            long rangeLength = Math.min(1024 * 1024, end - start + 1);
            return new ResourceRegion(resource, start, rangeLength);
        } else {
            long rangeLength = Math.min(1024 * 1024, contentLength);
            return new ResourceRegion(resource, 0, rangeLength);
        }
    }
}
