package ren.home.bingeAtHome.dao;

import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.util.ExternalConfigurationUtil;

import java.io.File;
import java.io.IOException;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.file.Paths;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class ImageDaoTest {

    private static final String videoRoot = "./videos";
    private static final String imageRoot = videoRoot + "/images";
    private static final String testFileImage = "best_mp4_for_test.mp4.webp";
    private static final String notExistsName = "not_exists.mp4.webp";

    @BeforeAll
    static void setUp() throws IOException, URISyntaxException {
        ExternalConfigurationUtil.init();
        URL resource = VideoDaoTest.class.getClassLoader().getResource(testFileImage);
        assert resource != null;
        FileUtils.copyFile(new File(resource.toURI()), new File(imageRoot + File.separator + testFileImage));
    }

    @AfterAll
    static void tearDown() throws IOException {
        FileUtils.forceDelete(new File(videoRoot));
    }

    @Autowired
    private ImageDao imageDao;

    @Test
    void readImage_whenExisting_thenPath() throws IOException {
        assertThat(imageDao.readImage(testFileImage)).isEqualTo(
                Paths.get(new File(ExternalConfigurationUtil.imageStorePath).getAbsolutePath(), testFileImage));
    }

    @Test
    void readImage_whenNotExisting_thenIoException() {
        assertThatThrownBy(() -> imageDao.readImage(notExistsName)).isInstanceOf(IOException.class);
    }
}