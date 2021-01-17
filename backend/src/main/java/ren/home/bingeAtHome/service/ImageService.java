package ren.home.bingeAtHome.service;


import org.springframework.core.io.UrlResource;
import ren.home.bingeAtHome.service.exception.ImageMissingException;

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
     * @return the image as a resource
     * @throws ImageMissingException thrown if the image is missing
     */
    UrlResource getPosterImage(String videoFileName) throws ImageMissingException;

}
