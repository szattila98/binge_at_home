package ren.home.bingeAtHome.service;

import ren.home.bingeAtHome.service.exception.TrackMissingException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.io.File;
import java.util.Map;

/**
 * Defines the basic operations with tracks.
 *
 * @author Attila Szőke
 */
public interface TrackService {

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
