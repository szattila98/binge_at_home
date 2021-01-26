package ren.home.bingeAtHome.dao;

import ren.home.bingeAtHome.model.VideoMetadata;

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
     * @param videoName the video name
     * @return the metadata
     * @throws IOException thrown when the metadata does not exist or cannot be read
     */
    VideoMetadata readMetadata(String videoName) throws IOException;

    /**
     * Saves a metadata.
     *
     * @param videoName     the file name
     * @param videoMetadata the metadata
     * @return the video name
     * @throws IOException thrown when the metadata cannot be saved
     */
    String saveMetadata(String videoName, VideoMetadata videoMetadata) throws IOException;

}
