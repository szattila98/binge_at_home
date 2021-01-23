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
import org.springframework.core.io.UrlResource;
import org.springframework.core.io.support.ResourceRegion;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpRange;
import org.springframework.http.ResponseEntity;
import ren.home.bingeAtHome.dao.VideoDao;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.service.exception.TrackMissingException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.net.MalformedURLException;
import java.net.URL;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class VideoServiceImplTest {

    private static String TEST_VIDEO;
    private static String TEST_TRACK;
    private static String NON_EXISTENT_FILE;
    private static File TEST_VIDEO_FILE;
    private static File TEST_TRACK_FILE;
    private static UrlResource VIDEO_URL_RESOURCE;

    @TempDir
    static File tempDir;

    @BeforeAll
    static void setUp() throws Exception {
        ExternalConfig.test_init(tempDir);
        TEST_VIDEO = "best_mp4_for_test.mp4";
        TEST_TRACK = "best_mp4_for_test.mp4-ENG.vtt";
        NON_EXISTENT_FILE = "no_such.file";
        TEST_VIDEO_FILE = new File(
                ExternalConfig.VIDEO_STORE_PATH + File.separator + TEST_VIDEO);
        TEST_TRACK_FILE = new File(
                ExternalConfig.TRACK_STORE_PATH + File.separator + TEST_TRACK);
        VIDEO_URL_RESOURCE = new UrlResource("file:" + TEST_VIDEO_FILE.getAbsolutePath());
        URL videoResource = VideoServiceImplTest.class.getClassLoader().getResource(TEST_VIDEO);
        URL trackResource = VideoServiceImplTest.class.getClassLoader().getResource(TEST_TRACK);
        assert videoResource != null && trackResource != null;
        FileUtils.copyFile(new File(videoResource.toURI()), TEST_VIDEO_FILE);
        FileUtils.copyFile(new File(trackResource.toURI()), TEST_TRACK_FILE);
    }

    @AfterAll
    static void tearDown() throws Exception {
        FileUtils.forceDelete(new File(ExternalConfig.VIDEO_STORE_PATH));
    }

    @Mock
    private VideoDao videoDao;

    @InjectMocks
    private VideoServiceImpl videoService;

    @Test
    void getAllVideos_returnsTestMp4AndNotReturnsMissingFileInVideoList() throws Exception {
        File notExists = new File(NON_EXISTENT_FILE);
        Video testVideo = new Video(TEST_VIDEO_FILE);

        Mockito.when(videoDao.findAllVideoFiles())
                .thenReturn(Lists.newArrayList(TEST_VIDEO_FILE, notExists));

        List<Video> videos = videoService.getAllVideos();

        // set it to null as on some OS's, it just puts a new Date there
        videos.get(0).setLastAccessed(null);
        testVideo.setLastAccessed(null);

        assertThat(videos).hasSize(1);
        assertThat(videos.get(0)).isEqualTo(testVideo);
    }

    @Test
    void prepareContent_whenRangeNotNullAndCorrectSize_returnCorrectResponseEntity() throws Exception {
        int expectedStatus = 206;
        long rangeStart = 0;
        long rangeEnd = 5000;
        long rangeLength = 5001;
        HttpHeaders httpHeaders = new HttpHeaders();
        httpHeaders.setRange(Lists.list(HttpRange.createByteRange(rangeStart, rangeEnd)));

        Mockito.when(videoDao.findResourceByName(TEST_VIDEO)).thenReturn(VIDEO_URL_RESOURCE);

        ResponseEntity<ResourceRegion> re = videoService.prepareContent(TEST_VIDEO, httpHeaders);
        assertThat(re.getStatusCodeValue()).isEqualTo(expectedStatus);
        assertThat(Objects.requireNonNull(re.getBody()).getResource()).isEqualTo(VIDEO_URL_RESOURCE);
        assertThat(re.getBody().getCount()).isEqualTo(rangeLength);
        assertThat(re.getBody().getPosition()).isEqualTo(rangeStart);
    }

    @Test
    void prepareContent_whenRangeNotNullAndTooLarge_returnCorrectRegion() throws Exception {
        int expectedStatus = 206;
        long rangeStart = 0;
        long rangeEnd = 1024 * 1024 + 1;
        long rangeLength = 1024 * 1024;
        HttpHeaders httpHeaders = new HttpHeaders();
        httpHeaders.setRange(Lists.list(HttpRange.createByteRange(rangeStart, rangeEnd)));

        Mockito.when(videoDao.findResourceByName(TEST_VIDEO)).thenReturn(VIDEO_URL_RESOURCE);

        ResponseEntity<ResourceRegion> re = videoService.prepareContent(TEST_VIDEO, httpHeaders);
        assertThat(re.getStatusCodeValue()).isEqualTo(expectedStatus);
        assertThat(Objects.requireNonNull(re.getBody()).getResource()).isEqualTo(VIDEO_URL_RESOURCE);
        assertThat(re.getBody().getCount()).isEqualTo(rangeLength);
        assertThat(re.getBody().getPosition()).isEqualTo(rangeStart);
    }

    @Test
    void prepareContent_whenNullRange_returnPreDefinedRegion() throws Exception {
        int expectedStatus = 206;
        long maxRangeLength = 1024 * 1024;
        HttpHeaders httpHeaders = new HttpHeaders();

        Mockito.when(videoDao.findResourceByName(TEST_VIDEO)).thenReturn(VIDEO_URL_RESOURCE);

        ResponseEntity<ResourceRegion> re = videoService.prepareContent(TEST_VIDEO, httpHeaders);
        assertThat(re.getStatusCodeValue()).isEqualTo(expectedStatus);
        assertThat(Objects.requireNonNull(re.getBody()).getResource()).isEqualTo(VIDEO_URL_RESOURCE);
        assertThat(re.getBody().getCount()).isEqualTo(maxRangeLength);
    }

    @Test
    void prepareContent_whenNotExistingVideo_throwException() throws Exception {
        Mockito.when(videoDao.findResourceByName(NON_EXISTENT_FILE)).thenThrow(new MalformedURLException());

        assertThatThrownBy(() -> videoService.prepareContent(NON_EXISTENT_FILE, null)).isInstanceOf(VideoMissingException.class);
    }

    @Test
    void getTrackInfo_whenExistingVideo_thenThrowCorrectMap() throws Exception {
        Map<String, String> expectedMap = new HashMap<>();
        expectedMap.put("ENG", "best_mp4_for_test.mp4-ENG.vtt");

        Mockito.when(videoDao.getVideoFile(TEST_VIDEO)).thenReturn(TEST_VIDEO_FILE);
        Mockito.when(videoDao.getTrackFiles(TEST_VIDEO)).thenReturn(Lists.newArrayList(new File(TEST_TRACK)));

        assertThat(videoService.getTrackInfo(TEST_VIDEO)).isEqualTo(expectedMap);
    }

    @Test
    void getTrackInfo_whenNotExistingVideo_throwException() {
        Mockito.when(videoDao.getVideoFile(NON_EXISTENT_FILE)).thenReturn(new File(NON_EXISTENT_FILE));

        assertThatThrownBy(() -> videoService.getTrackInfo(NON_EXISTENT_FILE)).isInstanceOf(VideoMissingException.class);
    }

    @Test
    void getTrack_whenExistingTrack_thenCorrectTrackReturned() throws TrackMissingException {
        Mockito.when(videoDao.readTrack(TEST_TRACK)).thenReturn(TEST_TRACK_FILE);

        assertThat(videoService.getTrack(TEST_TRACK)).isEqualTo(TEST_TRACK_FILE);
    }

    @Test
    void getTrack_whenNotExistingTrack_thenException() {
        String notExistsTrack = "no_such_track.vtt";

        Mockito.when(videoDao.readTrack(notExistsTrack)).thenReturn(new File(notExistsTrack));

        assertThatThrownBy(() -> videoService.getTrack(notExistsTrack)).isInstanceOf(TrackMissingException.class);
    }

}