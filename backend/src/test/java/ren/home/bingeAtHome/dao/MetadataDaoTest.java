package ren.home.bingeAtHome.dao;

import org.apache.commons.io.FileUtils;
import org.assertj.core.util.Lists;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.model.Metadata;

import java.io.File;
import java.io.IOException;
import java.net.URISyntaxException;
import java.net.URL;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class MetadataDaoTest {

    private static final String testFile = "best_mp4_for_test.mp4";
    private static final String testFileMetadata = "best_mp4_for_test.mp4.json";
    private static final String notExistsName = "not_exists.mp4";
    private static final String videoRoot = "./videos";
    private static final String metadataRoot = videoRoot + "/metadata";
    private static final String props = "./config.properties";

    @BeforeAll
    static void setUp() throws URISyntaxException, IOException {
        ExternalConfigurationUtil.init();
        URL resource1 = VideoDaoTest.class.getClassLoader().getResource(testFile);
        URL resource2 = VideoDaoTest.class.getClassLoader().getResource(testFileMetadata);
        assert resource1 != null;
        assert resource2 != null;
        FileUtils.copyFile(new File(resource1.toURI()), new File(videoRoot + "/" + testFile));
        FileUtils.copyFile(new File(resource2.toURI()), new File(metadataRoot + "/" + testFileMetadata));
    }

    @AfterAll
    static void tearDown() throws IOException {
        FileUtils.forceDelete(new File(videoRoot));
    }

    @Autowired
    private MetadataDao metadataDao;

    @Test
    void init_checkWhetherPropsAndVideoRootCreated() {
        assertThat(new File(props).exists()).isTrue();
        assertThat(new File(videoRoot).exists()).isTrue();
    }

    @Test
    void readMetadata_whenExisting_thenCorrectReturn() throws IOException {
        List<String> tags = Lists.list("Give", "You", "Up");
        List<String> captions = Lists.list("Let", "You", "Down");
        Metadata shouldBeMetadata = new Metadata("Never", "Gonna", tags, "Never Gonna", captions);

        metadataDao.saveMetadata(testFile, shouldBeMetadata);
        Metadata readMetadata = metadataDao.readMetadata(testFile);

        assertThat(readMetadata).isEqualTo(shouldBeMetadata);
    }

    @Test
    void readMetadata_whenNotExisting_thenException() {
        assertThatThrownBy(() -> metadataDao.readMetadata(notExistsName)).isInstanceOf(IOException.class);
    }

    @Test
    void saveMetadata_correctSave() throws IOException, InterruptedException {
        List<String> tags = Lists.list("Give", "You", "Up");
        List<String> captions = Lists.list("Let", "You", "Down");
        Metadata shouldBeMetadata = new Metadata("Always", "Gonna", tags, "Always Gonna", captions);

        metadataDao.saveMetadata(testFile, shouldBeMetadata);
        Metadata readMetadata = metadataDao.readMetadata(testFile);

        assertThat(readMetadata).isEqualTo(shouldBeMetadata);
        assertThat(new File(metadataRoot + File.separator + testFile + ".json").exists()).isTrue();
    }
}