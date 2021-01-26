package ren.home.bingeAtHome.controller;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.commons.io.FileUtils;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.mockito.Mockito;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest;
import org.springframework.boot.test.mock.mockito.MockBean;
import org.springframework.http.MediaType;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.MvcResult;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;
import ren.home.bingeAtHome.controller.util.ErrorMsgJsonCreator;
import ren.home.bingeAtHome.model.Video;
import ren.home.bingeAtHome.model.VideoMetadata;
import ren.home.bingeAtHome.service.TrackService;
import ren.home.bingeAtHome.service.exception.TrackMissingException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.net.URL;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;

import static org.assertj.core.api.Assertions.assertThat;

@WebMvcTest(TrackController.class)
class TrackControllerTest {

    private final ObjectMapper objectMapper = new ObjectMapper();
    private static final Video TEST_VIDEO = new Video(
            "mock_vid.mp4", new Date(), new Date(), 60000L, 60000L, ".mp4", new VideoMetadata());
    private static final String NOT_EXISTING_FILE = "no_such.file";
    private static final String TRACK_INFO_REQUEST_URI = "/api/track/info/";
    private static final String TRACK_REQUEST_URI = "/api/track/";

    @Autowired
    private MockMvc mockMvc;

    @MockBean
    private TrackService service;

    @TempDir
    static File tempDir;

    @BeforeAll
    static void init() {
        ExternalConfig.test_init(tempDir);
    }

    @Test
    void getTrackInfo_whenExistingTrack_thenReturnCorrectInfoJson() throws Exception {
        String uri = TRACK_INFO_REQUEST_URI + TEST_VIDEO.getFileName();
        String testTrack = "best_mp4_for_test.mp4-ENG.vtt";
        String filepath = ExternalConfig.TRACK_STORE_PATH + File.separator + testTrack;
        URL resource = TrackControllerTest.class.getClassLoader().getResource(testTrack);
        assert resource != null;
        File trackFile = new File(filepath);
        FileUtils.copyFile(new File(resource.toURI()), trackFile);
        Map<String, String> trackInfo = new HashMap<>();
        trackInfo.put("ENG", testTrack);

        Mockito.when(service.getTrackInfo(TEST_VIDEO.getFileName())).thenReturn(trackInfo);

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri)
                .accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo(objectMapper.writeValueAsString(trackInfo));
    }

    @Test
    void getTrackInfo_whenNotExistingVideo_thenReturnNotFoundAndCorrectMsg() throws Exception {
        String uri = TRACK_INFO_REQUEST_URI + NOT_EXISTING_FILE;

        Mockito.when(service.getTrackInfo(NOT_EXISTING_FILE)).thenThrow(new VideoMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri)
                .accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo(ErrorMsgJsonCreator.get(new VideoMissingException()));
    }

    @Test
    void getTrack_whenExistingTrack_thenReturnCorrectVtt() throws Exception {
        String testTrack = "best_mp4_for_test.mp4-ENG.vtt";
        String uri = TRACK_REQUEST_URI + testTrack;
        String filepath = ExternalConfig.TRACK_STORE_PATH + File.separator + testTrack;
        URL resource = TrackControllerTest.class.getClassLoader().getResource(testTrack);
        assert resource != null;
        File trackFile = new File(filepath);
        FileUtils.copyFile(new File(resource.toURI()), trackFile);


        Mockito.when(service.getTrack(testTrack)).thenReturn(trackFile);

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri)
                .accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        assertThat(mvcResult.getResponse().getContentLengthLong()).isEqualTo(trackFile.length());
    }

    @Test
    void getTrack_whenNotExistingTrack_thenReturnNotFoundAndCorrectMsg() throws Exception {
        String uri = TRACK_REQUEST_URI + NOT_EXISTING_FILE;

        Mockito.when(service.getTrack(NOT_EXISTING_FILE)).thenThrow(new TrackMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(uri)
                .accept(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo(ErrorMsgJsonCreator.get(new TrackMissingException()));

    }

}