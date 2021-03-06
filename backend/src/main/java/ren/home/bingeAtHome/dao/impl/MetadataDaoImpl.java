package ren.home.bingeAtHome.dao.impl;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.springframework.stereotype.Component;
import ren.home.bingeAtHome.dao.MetadataDao;
import ren.home.bingeAtHome.model.VideoMetadata;
import ren.home.bingeAtHome.util.ExternalConfig;

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
    private static final String METADATA_EXT = ".json";

    /**
     * {@inheritDoc}
     */
    @Override
    public VideoMetadata readMetadata(String videoName) throws IOException {
        return mapper.readValue(new File(ExternalConfig.METADATA_STORE_PATH, videoName + METADATA_EXT), VideoMetadata.class);
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public String saveMetadata(String videoName, VideoMetadata videoMetadata) throws IOException {
        mapper.writeValue(new File(ExternalConfig.METADATA_STORE_PATH, videoName + METADATA_EXT), videoMetadata);
        return videoName;
    }
}
