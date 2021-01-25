package ren.home.bingeAtHome.dao.impl;

import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.dao.ImageDao;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.io.IOException;
import java.net.URL;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class ImageDaoImplTest {

    @TempDir
    static File tempDir;

    @BeforeAll
    static void setUp() {
        ExternalConfig.test_init(tempDir);
    }

    @Autowired
    private ImageDao dao;

    @Test
    void readImage_whenExisting_thenPath() throws Exception {
        String testImage = "best_mp4_for_test.mp4.webp";
        File testImageFile = new File(ExternalConfig.IMAGE_STORE_PATH + File.separator + testImage);

        URL imageResource = VideoDaoImplTest.class.getClassLoader().getResource(testImage);
        assert imageResource != null;
        FileUtils.copyFile(new File(imageResource.toURI()), testImageFile);

        assertThat(FileUtils.readFileToByteArray(dao.readImage(testImage))).isEqualTo(FileUtils.readFileToByteArray(testImageFile));
    }

    @Test
    void readImage_whenNotExisting_thenException() {
        String nonExistentImage = "not_exists.mp4.webp";

        assertThatThrownBy(() -> dao.readImage(nonExistentImage)).isInstanceOf(IOException.class);
    }
}