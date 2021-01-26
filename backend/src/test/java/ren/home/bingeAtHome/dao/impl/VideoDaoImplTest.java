package ren.home.bingeAtHome.dao.impl;

import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.core.io.Resource;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.net.URL;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;

@SpringBootTest
class VideoDaoImplTest {

    private static final String TEST_VIDEO_MP4 = "best_mp4_for_test.mp4";
    private static File TEST_VIDEO_FILE_MP4;
    private static File TEST_VIDEO_FILE_WEBM;

    @TempDir
    static File tempDir;

    @BeforeAll
    static void setUp() throws Exception {
        ExternalConfig.test_init(tempDir);
        String testVideoMkv = "best_mkv_for_test.mkv";
        String testVideoWebm = "best_webm_for_test.webm";
        TEST_VIDEO_FILE_MP4 = new File(ExternalConfig.VIDEO_STORE_PATH + File.separator + TEST_VIDEO_MP4);
        File testVideoFileMkv =
                new File(ExternalConfig.VIDEO_STORE_PATH + File.separator + testVideoMkv);
        TEST_VIDEO_FILE_WEBM =
                new File(ExternalConfig.VIDEO_STORE_PATH + File.separator + testVideoWebm);
        URL resource1 = VideoDaoImplTest.class.getClassLoader().getResource(TEST_VIDEO_MP4);
        URL resource6 = VideoDaoImplTest.class.getClassLoader().getResource(testVideoMkv);
        URL resource5 = VideoDaoImplTest.class.getClassLoader().getResource(testVideoWebm);
        assert resource1 != null && resource5 != null && resource6 != null;
        FileUtils.copyFile(new File(resource1.toURI()), TEST_VIDEO_FILE_MP4);
        FileUtils.copyFile(new File(resource6.toURI()), testVideoFileMkv);
        FileUtils.copyFile(new File(resource5.toURI()), TEST_VIDEO_FILE_WEBM);
    }

    @Autowired
    private VideoDao videoDao;

    @Test
    void findAllVideoFiles_returnsMp4AndWebmOnly() throws Exception {
        List<File> videos = videoDao.findAllVideoFiles();
        File mp4 = new File("");
        File webm = new File("");

        assertThat(videos).hasSize(2);
        for (File video : videos) {
            if (video.getName().endsWith(".mp4")) {
                mp4 = video;
            }
            if (video.getName().endsWith(".webm")) {
                webm = video;
            }
        }
        assertThat(FileUtils.readFileToByteArray(mp4)).isEqualTo(FileUtils.readFileToByteArray(TEST_VIDEO_FILE_MP4));
        assertThat(FileUtils.readFileToByteArray(webm)).isEqualTo(FileUtils.readFileToByteArray(TEST_VIDEO_FILE_WEBM));
    }

    @Test
    void getVideoFile_returnsFile() throws Exception {
        assertThat(FileUtils.readFileToByteArray(videoDao.getVideoFile(TEST_VIDEO_MP4))).isEqualTo(FileUtils.readFileToByteArray(TEST_VIDEO_FILE_MP4));
    }

    @Test
    void findResourceByName_whenExists_thenReturnCorrectResource() throws Exception {
        Resource foundResource = videoDao.findResourceByName(TEST_VIDEO_MP4);

        assertThat(foundResource.exists()).isTrue();
        assertThat(FileUtils.readFileToByteArray(foundResource.getFile())).isEqualTo(FileUtils.readFileToByteArray(TEST_VIDEO_FILE_MP4));
    }

}