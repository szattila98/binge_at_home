package ren.home.bingeAtHome.service;

import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.HttpHeaders;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.util.List;

/**
 * Defines the basic operations with videos.
 *
 * @author Attila Szőke
 */
public interface VideoService {

    /**
     * Gets all the videos stored.
     *
     * @return video list
     */
    List<Video> getAllVideos();


    /**
     * Gets a specific video.
     *
     * @param fileName the file name
     * @return the video
     * @throws VideoMissingException the video missing exception
     */
    Video getVideo(String fileName) throws VideoMissingException;

    /**
     * Prepares video content in a response entity for streaming.
     *
     * @param videoName name of the video
     * @param headers   headers of the request, among them the range header is relevant
     * @return ResponseEntity with the requested range of bytes and standard headers
     * @throws VideoMissingException thrown when there is no such video in the video store
     */
    ResourceRegion prepareContent(String videoName, HttpHeaders headers) throws VideoMissingException;

}
