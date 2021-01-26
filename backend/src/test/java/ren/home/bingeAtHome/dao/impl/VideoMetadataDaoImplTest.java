package ren.home.bingeAtHome.dao.impl;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.commons.io.FileUtils;
import org.assertj.core.util.Lists;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.dao.MetadataDao;
import ren.home.bingeAtHome.model.VideoMetadata;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.io.IOException;
import java.net.URL;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class VideoMetadataDaoImplTest {

    private static final String TEST_VIDEO = "best_mp4_for_test.mp4";
    private static final String TEST_METADATA = "best_mp4_for_test.mp4.json";
    private static final List<String> TAGS = Lists.newArrayList("Give", "You", "Up");

    @TempDir
    static File tempDir;

    @BeforeAll
    static void setUp() {
        ExternalConfig.test_init(tempDir);
    }

    @Autowired
    private MetadataDao dao;

    @Test
    void readMetadata_whenExisting_thenReturnsCorrectMetadata() throws Exception {
        File testVideoFile = new File(ExternalConfig.VIDEO_STORE_PATH + File.separator + TEST_VIDEO);
        File testMetadataFile = new File(ExternalConfig.METADATA_STORE_PATH + File.separator + TEST_METADATA);
        VideoMetadata shouldBeVideoMetadata = new VideoMetadata("Never", "Gonna", TAGS);

        URL videoResource = VideoDaoImplTest.class.getClassLoader().getResource(TEST_VIDEO);
        URL metadataResource = VideoDaoImplTest.class.getClassLoader().getResource(TEST_METADATA);
        assert videoResource != null && metadataResource != null;
        FileUtils.copyFile(new File(videoResource.toURI()), testVideoFile);
        FileUtils.copyFile(new File(metadataResource.toURI()), testMetadataFile);

        //dao.saveMetadata(TEST_VIDEO, shouldBeMetadata);
        VideoMetadata readVideoMetadata = dao.readMetadata(TEST_VIDEO);

        assertThat(readVideoMetadata).isEqualTo(shouldBeVideoMetadata);
    }

    @Test
    void readMetadata_whenNotExisting_thenException() {
        String notExistingVideo = "not_exists.mp4";

        assertThatThrownBy(() -> dao.readMetadata(notExistingVideo)).isInstanceOf(IOException.class);
    }

    @Test
    void saveMetadata_correctSave() throws Exception {
        VideoMetadata shouldBeVideoMetadata = new VideoMetadata("Always", "Gonna", TAGS);
        File shouldBeMetadataFile =
                new File(ExternalConfig.METADATA_STORE_PATH + File.separator + TEST_METADATA);

        assertThat(dao.saveMetadata(TEST_VIDEO, shouldBeVideoMetadata)).isEqualTo(TEST_VIDEO);
        assertThat(shouldBeMetadataFile.exists()).isTrue();
        assertThat(new ObjectMapper().readValue(shouldBeMetadataFile, VideoMetadata.class)).isEqualTo(shouldBeVideoMetadata);
    }
}