package ren.home.bingeAtHome.service.impl;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import ren.home.bingeAtHome.dao.MetadataDao;
import ren.home.bingeAtHome.model.Metadata;
import ren.home.bingeAtHome.service.MetadataService;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;

import java.io.IOException;

/**
 * Base implementation of MetadataService.
 *
 * @author Attila Szőke
 */
@Service
public class MetadataServiceImpl implements MetadataService {

    private MetadataDao metadataDao;

    @Autowired
    public MetadataServiceImpl(MetadataDao metadataDao) {
        this.metadataDao = metadataDao;
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public void saveMetadata(String fileName, Metadata metadata) throws MetadataCannotBeSavedException {
        try {
            metadataDao.saveMetadata(fileName, metadata);
        } catch (IOException e) {
            throw new MetadataCannotBeSavedException();
        }
    }
}
