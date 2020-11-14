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

    private VideoDao videoDao;

    /**
     * Instantiates a new Video service.
     *
     * @param videoDao the video dao
     */
    @Autowired
    public VideoServiceImpl(VideoDao videoDao) {
        this.videoDao = videoDao;
    }


    /**
     * Default constructor for Video service.
     */
    public VideoServiceImpl() {
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public List<Video> getAllVideos() {
        List<Video> storedVideos = new ArrayList<>();
        for (File file : videoDao.findAllVideos()) {
            try {
                storedVideos.add(new Video(file));
            } catch (IOException e) {
                log.warn("Video fetched is now missing somehow!");
            }
        }
        Collections.sort(storedVideos);
        log.debug("Videos fetched: {}", storedVideos);
        return storedVideos;
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public ResponseEntity<ResourceRegion> prepareContent(String videoName, HttpHeaders headers) throws VideoMissingException {
        try {
            UrlResource resource = videoDao.findResourceByName(videoName);
            ResourceRegion region = resourceRegion(resource, headers);
            log.debug("Prepared video: {}", videoName);
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
