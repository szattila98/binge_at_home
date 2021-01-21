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
     * @param imageName the image name
     * @return the image
     * @throws IOException thrown when the image does not exist
     */
    Path readImage(String imageName) throws IOException;
}
