INSERT INTO catalog (path, display_name, short_desc, long_desc)
VALUES
    ('test_catalog', 'Action Movies', 'Get your adrenaline pumping', 'Experience heart-pounding action and thrilling adventures with our collection of action-packed movies.');
   
INSERT INTO metadata (size, duration, bitrate, width, height, framerate)
VALUES
    (901185, 30, '2000', '1280', '720', '30.0');

INSERT INTO video (path, display_name, short_desc, long_desc, catalog_id, sequent_id, metadata_id)
VALUES
    ('test_catalog/test_folder/test_video.webm', 'The Fast and the Furious', 'High-octane car chases', 'Buckle up for an adrenaline-fueled ride with fast cars and intense action in this iconic film series.', 1, NULL, 1),
    ('test_catalog/outer_test_folder/test_video.webm', 'Casablanca', 'A timeless romance', 'Experience the enduring love story set against the backdrop of World War II in the classic film "Casablanca."', 1, NULL, 1),
    ('test_catalog/test_folder/another_test_folder/another_test_video.webm', 'Toy Story', 'Toys come to life', 'Join Woody, Buzz Lightyear, and their toy friends in a heartwarming adventure filled with humor and imagination in "Toy Story."', 1, NULL, 1);
