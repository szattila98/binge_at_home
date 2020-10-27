package ren.home.homeflix.service;

import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.HttpHeaders;
import org.springframework.http.ResponseEntity;
import ren.home.homeflix.model.Video;
import ren.home.homeflix.service.exception.VideoMissingException;

import java.util.Set;

/**
 * Defines the basic operation of this application.
 *
 * @author Attila Szőke
 */
public interface VideoService {

    /**
     * Gets all the videos stored.
     *
     * @return video list
     */
    Set<Video> getAllVideos();

    /**
     * Prepares video content in a response entity for streaming.
     *
     * @param videoName name of the video
     * @param headers   headers of the request, among them the range header is relevant
     * @return ResponseEntity with the requested range of bytes and standard headers
     * @throws VideoMissingException thrown when there is no such video in the video store
     */
    ResponseEntity<ResourceRegion> prepareContent(String videoName, HttpHeaders headers) throws VideoMissingException;
}
