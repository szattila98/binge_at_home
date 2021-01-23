package ren.home.bingeAtHome.controller.impl;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.assertj.core.util.Lists;
import org.junit.jupiter.api.Test;
import org.mockito.Mockito;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest;
import org.springframework.boot.test.mock.mockito.MockBean;
import org.springframework.http.MediaType;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.MvcResult;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;
import ren.home.bingeAtHome.controller.MetadataController;
import ren.home.bingeAtHome.controller.dto.MetadataInput;
import ren.home.bingeAtHome.controller.impl.util.ErrorMsgJsonCreator;
import ren.home.bingeAtHome.model.Metadata;
import ren.home.bingeAtHome.service.MetadataService;
import ren.home.bingeAtHome.service.exception.MetadataCannotBeSavedException;
import ren.home.bingeAtHome.service.exception.VideoMissingException;

import static org.assertj.core.api.Assertions.assertThat;

@WebMvcTest(MetadataController.class)
public class MetadataControllerImplTest {

    private final ObjectMapper objectMapper = new ObjectMapper();
    private static final String URI = "/api/metadata";
    private static final String VALID_VIDEO = "mock_video.mp4";
    private static final Metadata VALID_METADATA = new Metadata("Never", "Gonna",
            Lists.newArrayList("Give", "You", "Up"));
    private static final MetadataInput VALID_METADATA_INPUT = new MetadataInput(VALID_VIDEO,
            VALID_METADATA.getVideoName(), VALID_METADATA.getDescription(), VALID_METADATA.getTags());

    @Autowired
    private MockMvc mockMvc;

    @MockBean
    private MetadataService service;

    @Test
    void saveMetadata_whenSaved_thenReturnOkAndStringReturned() throws Exception {
        Mockito.when(service.saveMetadata(VALID_VIDEO, VALID_METADATA)).thenReturn(VALID_VIDEO);

        MvcResult mvcResult =
                mockMvc.perform(MockMvcRequestBuilders.post(URI)
                        .content(objectMapper.writeValueAsString(VALID_METADATA_INPUT))
                        .accept(MediaType.APPLICATION_JSON)
                        .contentType(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(200);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo(VALID_VIDEO);
    }

    @Test
    void saveMetadata_whenVideoNotFound_thenReturnNotFound() throws Exception {
        Mockito.when(service.saveMetadata(VALID_VIDEO, VALID_METADATA)).thenThrow(new VideoMissingException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.post(URI)
                .content(objectMapper.writeValueAsString(VALID_METADATA_INPUT))
                .accept(MediaType.APPLICATION_JSON)
                .contentType(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(404);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo(ErrorMsgJsonCreator.get(new VideoMissingException()));
    }

    @Test
    void saveMetadata_whenCannotBeSaved_thenReturnInternalServerError() throws Exception {
        Mockito.when(service.saveMetadata(VALID_VIDEO, VALID_METADATA)).thenThrow(new MetadataCannotBeSavedException());

        MvcResult mvcResult = mockMvc.perform(MockMvcRequestBuilders.post(URI)
                .content(objectMapper.writeValueAsString(VALID_METADATA_INPUT))
                .accept(MediaType.APPLICATION_JSON)
                .contentType(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(500);
        assertThat(mvcResult.getResponse().getContentAsString()).isEqualTo(ErrorMsgJsonCreator.get(new MetadataCannotBeSavedException()));
    }

    @Test
    void saveMetadata_whenInvalidMetadataInput_thenReturnBadRequest() throws Exception {
        String invalid = "";
        Metadata invalidMetadata = new Metadata(invalid, invalid, Lists.newArrayList(invalid));
        MetadataInput invalidDto = new MetadataInput(invalid, invalidMetadata.getVideoName(),
                invalidMetadata.getDescription(), invalidMetadata.getTags());

        MvcResult mvcResult =
                mockMvc.perform(MockMvcRequestBuilders.post(URI)
                        .content(objectMapper.writeValueAsString(invalidDto))
                        .accept(MediaType.APPLICATION_JSON)
                        .contentType(MediaType.APPLICATION_JSON)).andReturn();

        assertThat(mvcResult.getResponse().getStatus()).isEqualTo(400);
    }

}
