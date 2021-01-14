package ren.home.bingeAtHome.dao;

import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.core.io.UrlResource;

import java.io.File;
import java.io.IOException;
import java.net.MalformedURLException;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.file.Paths;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;

@SpringBootTest
class VideoDaoTest {

    private static final String testFile = "best_mp4_for_test.mp4";
    private static final String videoRoot = "./videos";
    private static final String props = "./config.properties";

    @BeforeAll
    static void setUp() throws URISyntaxException, IOException {
        ExternalConfigurationUtil.init();
        URL resource = VideoDaoTest.class.getClassLoader().getResource(testFile);
        assert resource != null;
        FileUtils.copyFile(new File(resource.toURI()), new File(videoRoot + "/" + testFile));
    }

    @AfterAll
    static void tearDown() throws IOException {
        FileUtils.forceDelete(new File(videoRoot));
    }

    @Autowired
    private VideoDao videoDao;

    @Test
    void init_checkWhetherPropsAndVideoRootCreated() {
        assertThat(new File(props).exists()).isTrue();
        assertThat(new File(videoRoot).exists()).isTrue();
    }

    @Test
    void findAllVideoFiles_returnTestFileOnly() {
        List<File> videos = videoDao.findAllVideoFiles();
        assertThat(videos).hasSize(1);
        for (File file : videos) {
            assertThat(file.getName()).isEqualTo(testFile);
        }
    }

    @Test
    void findResourceByName_whenExists_thenCorrectResource() throws MalformedURLException {
        assertThat(videoDao.findResourceByName(testFile).exists()).isTrue();
        assertThat(videoDao.findResourceByName(testFile)).isEqualTo(new UrlResource(
                "file:" + Paths.get(new File(videoRoot).getAbsolutePath(), testFile)));
    }
}