package ren.home.bingeAtHome.service.impl;

import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import ren.home.bingeAtHome.dao.MetadataDao;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.model.Metadata;
import ren.home.bingeAtHome.service.MetadataService;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.io.IOException;

/**
 * Base implementation of MetadataService.
 *
 * @author Attila Szőke
 */
@Slf4j
@Service
public class MetadataServiceImpl implements MetadataService {

    private final MetadataDao metadataDao;
    private final VideoDao videoDao;

    /**
     * Instantiates a new Metadata service.
     *
     * @param metadataDao the metadata dao
     * @param videoDao    the video dao
     */
    @Autowired
    public MetadataServiceImpl(MetadataDao metadataDao, VideoDao videoDao) {
        this.metadataDao = metadataDao;
        this.videoDao = videoDao;
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public String saveMetadata(String fileName, Metadata metadata) throws MetadataCannotBeSavedException, VideoMissingException {
        try {
            videoDao.getVideoFile(fileName);
        } catch (IOException e) {
            log.debug("Video with this name does not exist: {}!", fileName);
            throw new VideoMissingException();
        }
        try {
            String videoName = metadataDao.saveMetadata(fileName, metadata);
            log.debug("Metadata {} saved for file: {}!", metadata, videoName);
            return videoName;
        } catch (IOException e) {
            throw new MetadataCannotBeSavedException();
        }
    }
}
