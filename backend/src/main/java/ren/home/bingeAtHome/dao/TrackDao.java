package ren.home.bingeAtHome.dao;

import java.io.File;
import java.io.IOException;
import java.util.List;

/**
 * Handles basic filesystem operations with tracks.
 *
 * @author Attila Szőke
 */
public interface TrackDao {

    /**
     * Retrieves the track files.
     *
     * @param videoName the name of the video
     * @return the path of the caption file
     */
    List<File> getTrackFiles(String videoName);

    /**
     * Retrieves a track file.
     *
     * @param trackName the name of the track
     * @return the track file
     */
    File readTrack(String trackName) throws IOException;
}
