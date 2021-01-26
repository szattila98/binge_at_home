package ren.home.bingeAtHome.controller;

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
import ren.home.bingeAtHome.service.ImageService;
import ren.home.bingeAtHome.service.exception.ImageMissingException;
import ren.home.bingeAtHome.util.ExternalConfig;

import java.io.File;
import java.net.URL;

import static org.assertj.core.api.Assertions.assertThat;

@WebMvcTest(ImageController.class)
class ImageControllerTest {

    private static final String FILE_NAME = "best_mp4_for_test.mp4";
    private static final String URI = "/api/poster/" + FILE_NAME;

    @Autowired
    private MockMvc mockMvc;

    @MockBean
    private ImageService service;

    @TempDir
    static File tempDir;

    @BeforeAll
    static void init() {
        ExternalConfig.test_init(tempDir);
    }

    @Test
    void getPoster_whenExisting_thenReturnCorrect() throws Exception {
        String image = "best_mp4_for_test.mp4.webp";
        String filepath = ExternalConfig.IMAGE_STORE_PATH + File.separator + image;
        File file = new File(filepath);
        URL imageResource = VideoControllerTest.class.getClassLoader().getResource(image);
        assert imageResource != null;
        FileUtils.copyFile(new File(imageResource.toURI()), file);

        Mockito.when(service.getPosterImage(FILE_NAME)).thenReturn(new File(filepath));

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(URI).accept(MediaType.ALL_VALUE)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        assertThat(mvcResult.getResponse().getContentLengthLong()).isEqualTo(file.length());
    }

    @Test
    void getPoster_whenNotExisting_thenReturnNotFoundAndErrorMsg() throws Exception {
        Mockito.when(service.getPosterImage(FILE_NAME)).thenThrow(new ImageMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.get(URI).accept(MediaType.ALL_VALUE)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo(ErrorMsgJsonCreator.get(new ImageMissingException()));
    }
}