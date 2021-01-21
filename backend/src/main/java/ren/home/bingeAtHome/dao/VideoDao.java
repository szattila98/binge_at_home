package ren.home.bingeAtHome.dao;

import org.springframework.core.io.UrlResource;

import java.io.File;
import java.net.MalformedURLException;
import java.nio.file.InvalidPathException;
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
    List<File> findAllVideoFiles();


    /**
     * Gets a video file.
     *
     * @param fileName the file name
     * @return the video file
     */
    File getVideoFile(String fileName);

    /**
     * Retrieves a file resource by it's name from the video store directory.
     *
     * @param name name of the file
     * @return the searched file
     * @throws MalformedURLException the malformed url exception
     * @throws InvalidPathException  the invalid path exception
     */
    UrlResource findResourceByName(String name) throws MalformedURLException, InvalidPathException;

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
    File readTrack(String trackName);
}
