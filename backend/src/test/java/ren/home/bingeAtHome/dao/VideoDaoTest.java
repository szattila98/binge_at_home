package ren.home.bingeAtHome.dao;

import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.core.io.UrlResource;
import ren.home.bingeAtHome.util.ExternalConfigurationUtil;

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
    private static final String testFileTrack = "best_mp4_for_test.mp4-ENG.vtt";
    private static final String badTrack1 = "bad_vttHUN.vtt";
    private static final String badTrack2 = "bad_vtt.srt";
    private static final String videoRoot = "./videos";
    private static final String props = "./config.properties";
    private static final File trackFile = new File(ExternalConfigurationUtil.trackStorePath + File.separator + testFileTrack);


    @BeforeAll
    static void setUp() throws URISyntaxException, IOException {
        ExternalConfigurationUtil.init();
        URL resource1 = VideoDaoTest.class.getClassLoader().getResource(testFile);
        URL resource2 = VideoDaoTest.class.getClassLoader().getResource(testFileTrack);
        URL resource3 = VideoDaoTest.class.getClassLoader().getResource(badTrack1);
        URL resource4 = VideoDaoTest.class.getClassLoader().getResource(badTrack2);
        assert resource1 != null;
        assert resource2 != null;
        assert resource3 != null;
        assert resource4 != null;
        FileUtils.copyFile(new File(resource1.toURI()), new File(videoRoot + File.separator + testFile));
        FileUtils.copyFile(new File(resource2.toURI()), new File(
                ExternalConfigurationUtil.trackStorePath + File.separator + testFileTrack));
        FileUtils.copyFile(new File(resource3.toURI()), new File(
                ExternalConfigurationUtil.trackStorePath + File.separator + badTrack1));
        FileUtils.copyFile(new File(resource4.toURI()), new File(
                ExternalConfigurationUtil.trackStorePath + File.separator + badTrack2));
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
    void getVideoFile_returnsFile() {
        assertThat(videoDao.getVideoFile(testFile)).isEqualTo(new File(videoRoot + File.separator + testFile));
    }

    @Test
    void findResourceByName_whenExists_thenCorrectResource() throws MalformedURLException {
        assertThat(videoDao.findResourceByName(testFile).exists()).isTrue();
        assertThat(videoDao.findResourceByName(testFile)).isEqualTo(new UrlResource(
                "file:" + Paths.get(new File(videoRoot).getAbsolutePath(), testFile)));
    }

    @Test
    void getTrackFiles_whenExisting_thenCorrectList() {
        assertThat(videoDao.getTrackFiles(testFile)).hasSize(1);
        assertThat(videoDao.getTrackFiles(testFile).get(0)).isEqualTo(trackFile);
    }

    @Test
    void getTrackFiles_whenNotExisting_thenEmptyList() {
        String notExistingVideo = "no_such.mp4";

        assertThat(videoDao.getTrackFiles(notExistingVideo)).isEmpty();
    }

    @Test
    void readTrack_returnsCorrectFile() {
        assertThat(videoDao.readTrack(testFileTrack).getAbsolutePath()).isEqualTo(trackFile.getAbsolutePath());
    }
}