package ren.home.bingeAtHome.service;

import org.apache.commons.io.FileUtils;
import org.assertj.core.util.Lists;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
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
import ren.home.bingeAtHome.service.impl.VideoServiceImpl;

import java.io.File;
import java.io.IOException;
import java.net.MalformedURLException;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class VideoServiceTest {

    private static final String testFile = "best_mp4_for_test.mp4";
    private static final String videoRoot = "./videos";
    private static final String notExistsName = "not_exists.mp4";
    private static final String testFileTrack = "best_mp4_for_test.mp4-ENG.vtt";

    @BeforeAll
    static void setUp() throws URISyntaxException, IOException {
        URL resource1 = VideoServiceTest.class.getClassLoader().getResource(testFile);
        URL resource2 = VideoServiceTest.class.getClassLoader().getResource(testFileTrack);
        assert resource1 != null;
        assert resource2 != null;
        FileUtils.copyFile(new File(resource1.toURI()), new File(videoRoot + File.separator + testFile));
        FileUtils.copyFile(new File(resource2.toURI()), new File(videoRoot + File.separator + testFileTrack));
    }

    @AfterAll
    static void tearDown() throws IOException {
        FileUtils.forceDelete(new File(videoRoot));
    }

    @Mock
    private VideoDao videoDao;

    @InjectMocks
    private final VideoService videoService = new VideoServiceImpl();

    @Test
    void getAllVideos_returnsTestMp4_notReturnsMissingFileInVideoList() {
        File file = new File(videoRoot + File.separator + testFile);
        File notExists = new File(videoRoot + File.separator + notExistsName);

        Mockito.when(videoDao.findAllVideoFiles()).thenReturn(Lists.newArrayList(file, notExists));

        List<Video> videos = videoService.getAllVideos();
        assertThat(videos).hasSize(1);
        for (Video video : videos) {
            assertThat(video.getFileName()).isEqualTo(testFile);
            assertThat(video.getMetadata()).isNull();
        }
    }

    @Test
    void prepareContent_whenRangeNotNullAndCorrectSize_returnCorrectResponseEntity() throws MalformedURLException, VideoMissingException {
        int expectedStatus = 206;
        long rangeStart = 0;
        long rangeEnd = 5000;
        long rangeLength = 5001;
        HttpHeaders httpHeaders = new HttpHeaders();
        httpHeaders.setRange(Lists.list(HttpRange.createByteRange(rangeStart, rangeEnd)));
        UrlResource resource = new UrlResource(
                "file:" + Paths.get(new File(videoRoot).getAbsolutePath(), testFile).toString());

        Mockito.when(videoDao.findResourceByName(testFile)).thenReturn(resource);

        ResponseEntity<ResourceRegion> re = videoService.prepareContent(testFile, httpHeaders);
        assertThat(re.getStatusCodeValue()).isEqualTo(expectedStatus);
        assertThat(Objects.requireNonNull(re.getBody()).getResource()).isEqualTo(resource);
        assertThat(re.getBody().getCount()).isEqualTo(rangeLength);
        assertThat(re.getBody().getPosition()).isEqualTo(rangeStart);
    }

    @Test
    void prepareContent_whenRangeNotNullAndTooLarge_returnCorrectRegion() throws MalformedURLException, VideoMissingException {
        int expectedStatus = 206;
        long rangeStart = 0;
        long rangeEnd = 1024 * 1024 + 1;
        long rangeLength = 1024 * 1024;
        HttpHeaders httpHeaders = new HttpHeaders();
        httpHeaders.setRange(Lists.list(HttpRange.createByteRange(rangeStart, rangeEnd)));
        UrlResource resource = new UrlResource(
                "file:" + Paths.get(new File(videoRoot).getAbsolutePath(), testFile).toString());

        Mockito.when(videoDao.findResourceByName(testFile)).thenReturn(resource);

        ResponseEntity<ResourceRegion> re = videoService.prepareContent(testFile, httpHeaders);
        assertThat(re.getStatusCodeValue()).isEqualTo(expectedStatus);
        assertThat(Objects.requireNonNull(re.getBody()).getResource()).isEqualTo(resource);
        assertThat(re.getBody().getCount()).isEqualTo(rangeLength);
        assertThat(re.getBody().getPosition()).isEqualTo(rangeStart);
    }

    @Test
    void prepareContent_whenNullRange_returnPreDefinedRegion() throws IOException, VideoMissingException {
        int expectedStatus = 206;
        long maxRangeLength = 1024 * 1024;
        HttpHeaders httpHeaders = new HttpHeaders();
        UrlResource resource = new UrlResource(
                "file:" + Paths.get(new File(videoRoot).getAbsolutePath(), testFile).toString());

        Mockito.when(videoDao.findResourceByName(testFile)).thenReturn(resource);

        ResponseEntity<ResourceRegion> re = videoService.prepareContent(testFile, httpHeaders);
        assertThat(re.getStatusCodeValue()).isEqualTo(expectedStatus);
        assertThat(Objects.requireNonNull(re.getBody()).getResource()).isEqualTo(resource);
        assertThat(re.getBody().getCount()).isEqualTo(maxRangeLength);
    }

    @Test
    void prepareContent_whenNotExistingVideo_throwException() throws MalformedURLException {
        Mockito.when(videoDao.findResourceByName(notExistsName)).thenThrow(new MalformedURLException());

        assertThatThrownBy(() -> videoService.prepareContent(notExistsName, null)).isInstanceOf(VideoMissingException.class);
    }

    @Test
    void getTrackInfo_whenExistingVideo_thenThrowCorrectMap() throws VideoMissingException {
        Map<String, String> expectedMap = new HashMap<>();
        expectedMap.put("ENG", "best_mp4_for_test.mp4-ENG.vtt");

        Mockito.when(videoDao.getVideoFile(testFile)).thenReturn(new File(videoRoot + File.separator + testFile));
        Mockito.when(videoDao.getTrackFiles(testFile)).thenReturn(Lists.newArrayList(new File(testFileTrack)));

        assertThat(videoService.getTrackInfo(testFile)).isEqualTo(expectedMap);
    }

    @Test
    void getTrackInfo_whenNotExistingVideo_throwException() {
        Mockito.when(videoDao.getVideoFile(notExistsName)).thenReturn(new File(notExistsName));

        assertThatThrownBy(() -> videoService.getTrackInfo(notExistsName)).isInstanceOf(VideoMissingException.class);
    }

    @Test
    void getTrack_whenExistingTrack_thenCorrectTrackReturned() throws TrackMissingException {
        Mockito.when(videoDao.readTrack(testFileTrack)).thenReturn(new File(videoRoot + File.separator + testFile));

        assertThat(videoService.getTrack(testFileTrack)).isEqualTo(new File(videoRoot + File.separator + testFile));
    }

    @Test
    void getTrack_whenNotExistingTrack_thenException() {
        String notExistsTrack = "no_such_track.vtt";

        Mockito.when(videoDao.readTrack(notExistsTrack)).thenReturn(new File(notExistsTrack));

        assertThatThrownBy(() -> videoService.getTrack(notExistsTrack)).isInstanceOf(TrackMissingException.class);
    }

}