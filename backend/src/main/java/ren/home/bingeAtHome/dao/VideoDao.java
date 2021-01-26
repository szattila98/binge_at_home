package ren.home.bingeAtHome.dao;

import org.springframework.core.io.UrlResource;

import java.io.File;
import java.io.IOException;
import java.net.MalformedURLException;
import java.nio.file.InvalidPathException;
import java.util.List;

/**
 * Handles basic filesystem operations with videos.
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
    File getVideoFile(String fileName) throws IOException;

    /**
     * Retrieves a file resource by it's resourceName from the video store directory.
     *
     * @param resourceName resourceName of the file
     * @return the searched file
     * @throws MalformedURLException the malformed url exception
     * @throws InvalidPathException  the invalid path exception
     */
    UrlResource findResourceByName(String resourceName) throws MalformedURLException, InvalidPathException;

}
