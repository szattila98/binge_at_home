package ren.home.bingeAtHome.dao;

import java.io.File;
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
     * @param imageName the image name
     * @return the image file
     * @throws IOException thrown when the image does not exist
     */
    File readImage(String imageName) throws IOException;
}
