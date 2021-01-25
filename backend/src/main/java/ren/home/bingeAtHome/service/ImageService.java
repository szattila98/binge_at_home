package ren.home.bingeAtHome.service;


import ren.home.bingeAtHome.service.exception.ImageMissingException;

import java.io.File;

/**
 * Defines the basic operations with images.
 *
 * @author Attila Szőke
 */
public interface ImageService {


    /**
     * Gets a poster image for a video.
     *
     * @param videoName the name of the video we want the poster of
     * @return the image file
     * @throws ImageMissingException thrown if the image is missing
     */
    File getPosterImage(String videoName) throws ImageMissingException;

}
