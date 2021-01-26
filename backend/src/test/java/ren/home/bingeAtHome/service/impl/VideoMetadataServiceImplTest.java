package ren.home.bingeAtHome.service.impl;

import org.assertj.core.util.Lists;
import org.junit.jupiter.api.Test;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.Mockito;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.dao.MetadataDao;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.model.VideoMetadata;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import java.io.File;
import java.io.IOException;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class VideoMetadataServiceImplTest {

    private static final String TEST_VIDEO = "best_mp4_for_test.mp4";
    private static final File TEST_VIDEO_FILE = new File(TEST_VIDEO);
    private static final VideoMetadata TEST_VIDEO_METADATA = new VideoMetadata(
            "Never", "Gonna", Lists.list("Up"));

    @Mock
    private VideoDao videoDao;
    @Mock
    private MetadataDao metadataDao;

    @InjectMocks
    private MetadataServiceImpl metadataService;

    @Test
    void saveMetadata_whenEverythingCorrect_thenReturnSavedFileName() throws Exception {
        Mockito.when(videoDao.getVideoFile(TEST_VIDEO)).thenReturn(TEST_VIDEO_FILE);
        Mockito.when(metadataDao.saveMetadata(TEST_VIDEO, TEST_VIDEO_METADATA)).thenReturn(TEST_VIDEO);

        assertThat(metadataService.saveMetadata(TEST_VIDEO, TEST_VIDEO_METADATA)).isEqualTo(TEST_VIDEO);
    }

    @Test
    void saveMetadata_whenFileNotExists_throwException() throws Exception {
        String nonExistentVideo = "no_such.mp4";

        Mockito.when(videoDao.getVideoFile(nonExistentVideo)).thenThrow(new IOException());

        assertThatThrownBy(
                () -> metadataService.saveMetadata(nonExistentVideo, TEST_VIDEO_METADATA))
                .isInstanceOf(VideoMissingException.class);
    }

    @Test
    void saveMetadata_whenIoException_throwException() throws Exception {
        Mockito.when(videoDao.getVideoFile(TEST_VIDEO)).thenReturn(TEST_VIDEO_FILE);
        Mockito.when(metadataDao.saveMetadata(TEST_VIDEO, TEST_VIDEO_METADATA))
                .thenThrow(new IOException());

        assertThatThrownBy(() -> metadataService.saveMetadata(TEST_VIDEO, TEST_VIDEO_METADATA))
                .isInstanceOf(MetadataCannotBeSavedException.class);
    }

}