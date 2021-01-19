package ren.home.bingeAtHome.service;

import org.junit.jupiter.api.Test;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.Mockito;
import org.springframework.boot.test.context.SpringBootTest;
import ren.home.bingeAtHome.dao.ImageDao;
import ren.home.bingeAtHome.service.exception.ImageMissingException;
import ren.home.bingeAtHome.service.impl.ImageServiceImpl;

import java.io.IOException;
import java.nio.file.Path;
import java.nio.file.Paths;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

@SpringBootTest
class ImageServiceTest {

    private static final String testFile = "best_mp4_for_test.mp4";
    private static final String notExistsName = "not_exists.mp4";
    private static final String ext = ".webp";

    @Mock
    private ImageDao imageDao;

    @InjectMocks
    private final ImageService imageService = new ImageServiceImpl(imageDao);

    @Test
    void getPosterImage_whenExisting_thenPath() throws IOException, ImageMissingException {
        String image = testFile + ext;
        Path path = Paths.get(image);

        Mockito.when(imageDao.readImage(image)).thenReturn(path);

        assertThat(imageService.getPosterImage(testFile)).isEqualTo(path);
    }

    @Test
    void getPosterImage_whenNotExisting_thenException() throws IOException {
        String missingImage = notExistsName + ext;

        Mockito.when(imageDao.readImage(missingImage)).thenThrow(new IOException());

        assertThatThrownBy(() -> imageService.getPosterImage(notExistsName)).isInstanceOf(ImageMissingException.class);
    }
}