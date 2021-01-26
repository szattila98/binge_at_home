package ren.home.bingeAtHome.dao.impl;

import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.dao.TrackDao;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.net.URL;

import static org.assertj.core.api.Assertions.assertThat;

@SpringBootTest
class TrackDaoImplTest {

    private static final String TEST_VIDEO_MP4 = "best_mp4_for_test.mp4";
    private static File TEST_TRACK_FILE;
    private static final String TEST_TRACK = "best_mp4_for_test.mp4-ENG.vtt";
    private static final String BAD_TRACK_1 = "bad_vttHUN.vtt";
    private static final String BAD_TRACK_2 = "bad_vtt.srt";

    @TempDir
    static File tempDir;

    @BeforeAll
    static void setUp() throws Exception {
        ExternalConfig.test_init(tempDir);
        TEST_TRACK_FILE = new File(ExternalConfig.TRACK_STORE_PATH + File.separator + TEST_TRACK);
        File BAD_TRACK_1_FILE = new File(ExternalConfig.TRACK_STORE_PATH + File.separator + BAD_TRACK_1);
        File BAD_TRACK_2_FILE = new File(ExternalConfig.TRACK_STORE_PATH + File.separator + BAD_TRACK_2);
        URL resource1 = VideoDaoImplTest.class.getClassLoader().getResource(TEST_VIDEO_MP4);
        URL resource2 = VideoDaoImplTest.class.getClassLoader().getResource(TEST_TRACK);
        URL resource3 = VideoDaoImplTest.class.getClassLoader().getResource(BAD_TRACK_1);
        URL resource4 = VideoDaoImplTest.class.getClassLoader().getResource(BAD_TRACK_2);
        assert resource1 != null && resource2 != null && resource3 != null && resource4 != null;
        FileUtils.copyFile(new File(resource2.toURI()), TEST_TRACK_FILE);
        FileUtils.copyFile(new File(resource3.toURI()), BAD_TRACK_1_FILE);
        FileUtils.copyFile(new File(resource4.toURI()), BAD_TRACK_2_FILE);
    }

    @Autowired
    private TrackDao trackDao;

    @Test
    void getTrackFiles_whenExisting_thenReturnCorrectList() throws Exception {
        assertThat(trackDao.getTrackFiles(TEST_VIDEO_MP4)).hasSize(1);
        assertThat(FileUtils.readFileToByteArray(trackDao.getTrackFiles(TEST_VIDEO_MP4).get(0))).isEqualTo(FileUtils.readFileToByteArray(TEST_TRACK_FILE));
    }

    @Test
    void getTrackFiles_whenNotExisting_thenReturnEmptyList() {
        String notExistingVideo = "no_such.mp4";

        assertThat(trackDao.getTrackFiles(notExistingVideo)).isEmpty();
    }

    @Test
    void readTrack_returnsCorrectFile() throws Exception {
        assertThat(FileUtils.readFileToByteArray(trackDao.readTrack(TEST_TRACK))).isEqualTo(FileUtils.readFileToByteArray(TEST_TRACK_FILE));
    }
}