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

    private MetadataDao metadataDao;
    private VideoDao videoDao;

    public MetadataServiceImpl() {
    }

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
        if (!videoDao.getVideoFile(fileName).exists()) {
            throw new VideoMissingException();
        }
        try {
            log.debug("Metadata {} saved for file: {}!", metadata, fileName);
            return metadataDao.saveMetadata(fileName, metadata);
        } catch (IOException e) {
            throw new MetadataCannotBeSavedException();
        }
    }
}
