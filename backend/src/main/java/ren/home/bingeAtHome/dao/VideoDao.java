package ren.home.bingeAtHome.dao;

import java.io.File;
import java.util.List;

/**
 * Handles basic filesystem operations with the videos.
 *
 * @author Attila Szőke
 */
public interface VideoDao {

    /**
     * Retrieves all videos from the video store directory.
     *
     * @return the list of files
     */
    List<File> findAllVideos();

    /**
     * Retrieves a file by it's name from the video store directory.
     *
     * @param name name of the file
     * @return the searched file
     */
    File findByName(String name);
}
