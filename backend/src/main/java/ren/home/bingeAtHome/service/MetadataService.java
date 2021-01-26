package ren.home.bingeAtHome.service;

import ren.home.bingeAtHome.model.VideoMetadata;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

/**
 * Defines the basic operations with metadata.
 *
 * @author Attila Szőke
 */
public interface MetadataService {

    /**
     * Adds metadata to a video.
     *
     * @param fileName      the file name
     * @param videoMetadata the metadata
     * @return file name
     * @throws MetadataCannotBeSavedException the metadata cannot be saved exception
     * @throws VideoMissingException          the video missing exception
     */
    String saveMetadata(String fileName, VideoMetadata videoMetadata) throws MetadataCannotBeSavedException, VideoMissingException;
}
