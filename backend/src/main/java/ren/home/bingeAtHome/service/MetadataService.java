package ren.home.bingeAtHome.service;

import ren.home.bingeAtHome.model.Metadata;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;

/**
 * Defines the basic operations with metadata.
 *
 * @author Attila Szőke
 */
public interface MetadataService {

    /**
     * Adds metadata to a video.
     *
     * @param fileName the file name
     * @param metadata the metadata
     */
    void saveMetadata(String fileName, Metadata metadata) throws MetadataCannotBeSavedException;
}
