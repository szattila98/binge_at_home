package ren.home.bingeAtHome.dao;

import org.springframework.core.io.UrlResource;

import java.io.IOException;

/**
 * Handles basic operations of image storage.
 *
 * @author Attila Szőke
 */
public interface ImageDao {

    /**
     * Gets an image.
     *
     * @param fileName the name of the image file
     * @return the image
     */
    UrlResource readImage(String fileName) throws IOException;
}
