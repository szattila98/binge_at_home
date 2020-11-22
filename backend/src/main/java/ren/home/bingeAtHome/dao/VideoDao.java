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
     * Retrieves a file resource by it's name from the video store directory.
     *
     * @param name name of the file
     * @return the searched file
     */
    UrlResource findResourceByName(String name) throws MalformedURLException, InvalidPathException;
}
