package ren.home.bingeAtHome.dao.impl;

import org.springframework.stereotype.Component;
import ren.home.bingeAtHome.dao.ImageDao;
import ren.home.bingeAtHome.util.ExternalConfigurationUtil;

import java.io.File;
import java.io.IOException;
import java.nio.file.Path;
import java.nio.file.Paths;

/**
 * Basic implementation of the ImageDao interface.
 *
 * @author Attila Szőke
 */
@Component
public class ImageDaoImpl implements ImageDao {

    /**
     * {@inheritDoc}
     */
    @Override
    public Path readImage(String imageId) throws IOException {
        Path image = Paths.get(new File(ExternalConfigurationUtil.imageStorePath).getAbsolutePath(), imageId);
        if (!image.toFile().exists()) throw new IOException();
        return image;
    }
}
