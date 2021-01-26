package ren.home.bingeAtHome.service.impl;

import org.assertj.core.util.Lists;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.Mockito;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.dao.TrackDao;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.service.exception.TrackMissingException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class TrackServiceImplTest {

    private static final String NON_EXISTENT_FILE = "no_such.file";
    private static final String TEST_TRACK = "best_mp4_for_test.mp4-ENG.vtt";
    private static final File TEST_TRACK_FILE = new File(TEST_TRACK);
    private static final String TEST_VIDEO = "best_mp4_for_test.mp4";
    private static File TEST_VIDEO_FILE;

    @TempDir
    static File tempDir;

    @BeforeAll
    static void setUp() {
        ExternalConfig.test_init(tempDir);
        TEST_VIDEO_FILE = new File(
                ExternalConfig.VIDEO_STORE_PATH + File.separator + TEST_VIDEO);
    }

    @Mock
    private TrackDao trackDao;

    @Mock
    private VideoDao videoDao;

    @InjectMocks
    private TrackServiceImpl trackService;

    @Test
    void getTrackInfo_whenExistingVideo_thenThrowCorrectMap() throws Exception {
        Map<String, String> expectedMap = new HashMap<>();
        expectedMap.put("ENG", "best_mp4_for_test.mp4-ENG.vtt");

        Mockito.when(videoDao.getVideoFile(TEST_VIDEO)).thenReturn(TEST_VIDEO_FILE);
        Mockito.when(trackDao.getTrackFiles(TEST_VIDEO)).thenReturn(Lists.newArrayList(new File(TEST_TRACK)));

        assertThat(trackService.getTrackInfo(TEST_VIDEO)).isEqualTo(expectedMap);
    }

    @Test
    void getTrackInfo_whenNotExistingVideo_throwException() throws Exception {
        Mockito.when(videoDao.getVideoFile(NON_EXISTENT_FILE)).thenThrow(new IOException());

        assertThatThrownBy(() -> trackService.getTrackInfo(NON_EXISTENT_FILE)).isInstanceOf(VideoMissingException.class);
    }

    @Test
    void getTrack_whenExistingTrack_thenCorrectTrackReturned() throws Exception {
        Mockito.when(trackDao.readTrack(TEST_TRACK)).thenReturn(TEST_TRACK_FILE);

        assertThat(trackService.getTrack(TEST_TRACK)).isEqualTo(TEST_TRACK_FILE);
    }

    @Test
    void getTrack_whenNotExistingTrack_thenException() throws Exception {
        String notExistsTrack = "no_such_track.vtt";

        Mockito.when(trackDao.readTrack(notExistsTrack)).thenThrow(new IOException());

        assertThatThrownBy(() -> trackService.getTrack(notExistsTrack)).isInstanceOf(TrackMissingException.class);
    }
}