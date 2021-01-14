package ren.home.bingeAtHome.service;

import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.Mockito;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.dao.MetadataDao;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;
import ren.home.bingeAtHome.service.impl.MetadataServiceImpl;

import java.io.File;
import java.io.IOException;
import java.net.URISyntaxException;
import java.net.URL;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class MetadataServiceTest {

    private static final String testFile = "best_mp4_for_test.mp4";
    private static final String videoRoot = "./videos";
    private static final String notExistsName = "not_exists.mp4";

    @BeforeAll
    static void setUp() throws URISyntaxException, IOException {
        URL resource = VideoServiceTest.class.getClassLoader().getResource(testFile);
        assert resource != null;
        FileUtils.copyFile(new File(resource.toURI()), new File(videoRoot + "/" + testFile));
    }

    @AfterAll
    static void tearDown() throws IOException {
        FileUtils.forceDelete(new File(videoRoot));
    }

    @Mock
    private VideoDao videoDao;
    @Mock
    private MetadataDao metadataDao;

    @InjectMocks
    private final MetadataService metadataService = new MetadataServiceImpl();

    @Test
    void saveMetadata_whenEverythingCorrect_thenReturnFileName() throws IOException, VideoMissingException, MetadataCannotBeSavedException {
        Mockito.when(videoDao.getVideoFile(testFile)).thenReturn(new File(videoRoot + File.separator + testFile));
        Mockito.when(metadataDao.saveMetadata(testFile, null)).thenReturn(testFile);

        assertThat(metadataService.saveMetadata(testFile, null)).isEqualTo(testFile);
    }

    @Test
    void saveMetadata_whenFileNotExists_throwException() {
        Mockito.when(videoDao.getVideoFile(notExistsName)).thenReturn(new File(notExistsName));

        assertThatThrownBy(() -> metadataService.saveMetadata(notExistsName, null)).isInstanceOf(VideoMissingException.class);
    }

    @Test
    void saveMetadata_whenIoException_throwException() throws IOException {
        Mockito.when(videoDao.getVideoFile(testFile)).thenReturn(new File(videoRoot + File.separator + testFile));
        Mockito.when(metadataDao.saveMetadata(testFile, null)).thenThrow(new IOException());

        assertThatThrownBy(() -> metadataService.saveMetadata(testFile, null)).isInstanceOf(MetadataCannotBeSavedException.class);
    }

}