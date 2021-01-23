package ren.home.bingeAtHome.service.impl;

import org.apache.commons.io.FileUtils;
import org.assertj.core.util.Lists;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.Mockito;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.dao.MetadataDao;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.model.Metadata;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.io.IOException;
import java.net.URL;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class MetadataServiceImplTest {

    private static final String TEST_VIDEO = "best_mp4_for_test.mp4";
    private static final File TEST_VIDEO_FILE = new File(
            ExternalConfig.VIDEO_STORE_PATH + File.separator + TEST_VIDEO);
    private static final Metadata TEST_METADATA = new Metadata(
            "Never", "Gonna", Lists.list("Up"));

    @TempDir
    static File tempDir;

    @BeforeAll
    static void setUp() throws Exception {
        ExternalConfig.test_init(tempDir);
        URL videoResource = VideoServiceImplTest.class.getClassLoader().getResource(TEST_VIDEO);
        assert videoResource != null;
        FileUtils.copyFile(new File(videoResource.toURI()), TEST_VIDEO_FILE);
    }

    @AfterAll
    static void tearDown() throws Exception {
        FileUtils.forceDelete(new File(ExternalConfig.VIDEO_STORE_PATH));
    }

    @Mock
    private VideoDao videoDao;
    @Mock
    private MetadataDao metadataDao;

    @InjectMocks
    private MetadataServiceImpl metadataService;

    @Test
    void saveMetadata_whenEverythingCorrect_thenReturnSavedFileName() throws Exception {
        Mockito.when(videoDao.getVideoFile(TEST_VIDEO)).thenReturn(TEST_VIDEO_FILE);
        Mockito.when(metadataDao.saveMetadata(TEST_VIDEO, TEST_METADATA)).thenReturn(TEST_VIDEO);

        assertThat(metadataService.saveMetadata(TEST_VIDEO, TEST_METADATA)).isEqualTo(TEST_VIDEO);
    }

    @Test
    void saveMetadata_whenFileNotExists_throwException() {
        String nonExistentVideo = "no_such.mp4";

        Mockito.when(videoDao.getVideoFile(nonExistentVideo)).thenReturn(new File(nonExistentVideo));

        assertThatThrownBy(
                () -> metadataService.saveMetadata(nonExistentVideo, TEST_METADATA))
                .isInstanceOf(VideoMissingException.class);
    }

    @Test
    void saveMetadata_whenIoException_throwException() throws Exception {
        Mockito.when(videoDao.getVideoFile(TEST_VIDEO)).thenReturn(TEST_VIDEO_FILE);
        Mockito.when(metadataDao.saveMetadata(TEST_VIDEO, TEST_METADATA))
                .thenThrow(new IOException());

        assertThatThrownBy(() -> metadataService.saveMetadata(TEST_VIDEO, TEST_METADATA))
                .isInstanceOf(MetadataCannotBeSavedException.class);
    }

}