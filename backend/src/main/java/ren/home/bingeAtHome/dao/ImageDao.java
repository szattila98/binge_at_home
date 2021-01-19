package ren.home.bingeAtHome.dao;

import java.io.IOException;
import java.nio.file.Path;

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
    Path readImage(String fileName) throws IOException;
}
