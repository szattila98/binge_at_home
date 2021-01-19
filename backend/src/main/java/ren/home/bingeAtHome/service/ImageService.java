package ren.home.bingeAtHome.service;


import ren.home.bingeAtHome.service.exception.ImageMissingException;

import java.nio.file.Path;

/**
 * Defines the basic operations with images.
 *
 * @author Attila Szőke
 */
public interface ImageService {


    /**
     * Gets a poster image for a video.
     *
     * @param videoFileName the name of the video we want the poster of
     * @return the image path
     * @throws ImageMissingException thrown if the image is missing
     */
    Path getPosterImage(String videoFileName) throws ImageMissingException;

}
