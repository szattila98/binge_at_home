package ren.home.bingeAtHome.dao.impl;

import org.apache.commons.io.FileUtils;
import org.springframework.core.io.UrlResource;
import org.springframework.stereotype.Component;
import ren.home.bingeAtHome.dao.ConfigUtil;
import ren.home.bingeAtHome.dao.VideoDao;

import java.io.File;
import java.net.MalformedURLException;
import java.nio.file.InvalidPathException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;


/**
 * Basic implementation of the VideoDao interface.
 *
 * @author Attila Szőke
 */
@Component
public class VideoDaoImpl implements VideoDao {

    /**
     * {@inheritDoc}
     */
    @Override
    public List<File> findAllVideoFiles() {
        return new ArrayList<>(FileUtils.listFiles(new File(ConfigUtil.videoStorePath), ConfigUtil.validExtensions, false));
    }

    /**
     * {@inheritDoc}
     */
    @Override
    public UrlResource findResourceByName(String name) throws MalformedURLException, InvalidPathException {
        return new UrlResource("file:" + Paths.get(new File(ConfigUtil.videoStorePath).getAbsolutePath(), name).toString());
    }
}
