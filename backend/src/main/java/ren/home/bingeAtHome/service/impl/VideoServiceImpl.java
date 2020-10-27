package ren.home.bingeAtHome.service.impl;

import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.io.UrlResource;
import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.*;
import org.springframework.stereotype.Service;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.service.VideoService;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.attribute.BasicFileAttributes;
import java.util.Date;
import java.util.HashSet;
import java.util.Set;

/**
 * Base implementation of VideoService.
 *
 * @author Attila Szőke
 */
@Slf4j
@Service
public class VideoServiceImpl implements VideoService {

    private final VideoDao videoDao;

    /**
     * Instantiates a new Homeflix service.
     *
     * @param videoDao the video dao
     */
    @Autowired
    public VideoServiceImpl(VideoDao videoDao) {
        this.videoDao = videoDao;
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public Set<Video> getAllVideos() {
        Set<Video> storedVideos = new HashSet<>();
        for (File file : videoDao.findAllVideos()) {
            try {
                storedVideos.add(getVideoAttributes(file));
            } catch (IOException e) {
                log.debug("Video fetched is now missing somehow!");
            }
        }
        log.debug("Videos fetched: {}", storedVideos);
        return storedVideos;
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public ResponseEntity<ResourceRegion> prepareContent(String videoName, HttpHeaders headers) throws VideoMissingException {
        Video video = getVideo(videoName);
        log.debug("Preparing video: {}", video);
        try {
            UrlResource resource = new UrlResource("file:" + video.getFullPath());
            ResourceRegion region = resourceRegion(resource, headers);
            return ResponseEntity.status(HttpStatus.PARTIAL_CONTENT)
                    .contentType(MediaTypeFactory
                            .getMediaType(resource)
                            .orElse(MediaType.APPLICATION_OCTET_STREAM))
                    .body(region);
        } catch (IOException e) {
            log.debug("Video fetched is now missing somehow!");
            throw new VideoMissingException();
        }
    }

    /**
     * Gets a video by its name.
     *
     * @param name queried video's name
     * @return the video
     */
    private Video getVideo(String name) throws VideoMissingException {
        File file = videoDao.findByName(name);
        try {
            return getVideoAttributes(file);
        } catch (IOException e) {
            log.debug("Video fetched is now missing somehow!");
            throw new VideoMissingException();
        }
    }

    /**
     * Returns a video object with it's attributes.
     *
     * @param file the file
     * @return the video object
     * @throws IOException thrown when the video is missing
     */
    private Video getVideoAttributes(File file) throws IOException {
        BasicFileAttributes attr = Files.readAttributes(file.toPath(), BasicFileAttributes.class);
        return new Video(file.getName(), new Date(attr.creationTime().toMillis()),
                new Date(attr.lastAccessTime().toMillis()), attr.size(), file.getAbsolutePath());
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
