package ren.home.bingeAtHome.service;

import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.HttpHeaders;
import org.springframework.http.ResponseEntity;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.service.exception.TrackMissingException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.io.File;
import java.util.List;
import java.util.Map;

/**
 * Defines the basic operations of this application.
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
    ResponseEntity<ResourceRegion> prepareContent(String videoName, HttpHeaders headers) throws VideoMissingException;

    /**
     * Returns a key,value pair of track names and their language.
     *
     * @param videoName the video name
     * @return map of tracks
     */
    Map<String, String> getTrackInfo(String videoName) throws VideoMissingException;

    /**
     * Retrieves a track file.
     *
     * @param trackName name of the track file
     * @return the track file
     */
    File getTrack(String trackName) throws TrackMissingException;
}
