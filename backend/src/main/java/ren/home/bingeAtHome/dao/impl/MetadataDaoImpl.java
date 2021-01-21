package ren.home.bingeAtHome.dao.impl;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.springframework.stereotype.Component;
import ren.home.bingeAtHome.dao.MetadataDao;
import ren.home.bingeAtHome.model.Metadata;
import ren.home.bingeAtHome.util.ExternalConfigurationUtil;

import java.io.File;
import java.io.IOException;

/**
 * Basic implementation of the VideoDao interface.
 *
 * @author Attila Szőke
 */
@Component
public class MetadataDaoImpl implements MetadataDao {

    private final ObjectMapper mapper = new ObjectMapper();
    private static final String metadataExt = ".json";

    /**
     * {@inheritDoc}
     */
    @Override
    public Metadata readMetadata(String videoName) throws IOException {
        return mapper.readValue(new File(ExternalConfigurationUtil.metadataStorePath, videoName + metadataExt), Metadata.class);
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public String saveMetadata(String videoName, Metadata metadata) throws IOException {
        mapper.writeValue(new File(ExternalConfigurationUtil.metadataStorePath, videoName + metadataExt), metadata);
        return videoName;
    }
}
