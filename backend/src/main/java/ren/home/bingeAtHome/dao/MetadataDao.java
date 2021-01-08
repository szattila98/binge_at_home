package ren.home.bingeAtHome.dao;

import ren.home.bingeAtHome.model.Metadata;

import java.io.IOException;

/**
 * Handles basic operations of metadata storage.
 *
 * @author Attila Szőke
 */
public interface MetadataDao {

    /**
     * Reads a specific metadata.
     *
     * @param fileName the video name
     * @return the metadata
     */
    Metadata readMetadata(String fileName) throws IOException;

    /**
     * Saves a metadata.
     *
     * @param metadata the metadata
     */
    void saveMetadata(String fileName, Metadata metadata) throws IOException;

}
