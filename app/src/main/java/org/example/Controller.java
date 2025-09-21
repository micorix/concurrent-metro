package org.example;

import javafx.fxml.FXML;
import javafx.scene.layout.GridPane;
import javafx.scene.paint.Color;
import org.yaml.snakeyaml.Yaml;

import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

public class Controller {

    @FXML
    private GridPane board;

    private final List<Train> trains = new ArrayList<>();
    private final CoordinatesLock coordinatesLock = new CoordinatesLock();

    @FXML
    public void initialize() {
        Yaml yaml = new Yaml();
        InputStream inputStream = getClass().getResourceAsStream("/org/example/routes/basic_map_routes.yml");
        Map<String, List<Map<String, Object>>> config = yaml.load(inputStream);

        List<Map<String, Object>> trainConfigs = config.get("trains");
        for (Map<String, Object> trainConfig : trainConfigs) {
            String id = (String) trainConfig.get("id");
            Color color = Color.valueOf((String) trainConfig.get("color"));
            List<List<Integer>> pathList = (List<List<Integer>>) trainConfig.get("path");
            Coords[] path = new Coords[pathList.size()];
            for (int i = 0; i < pathList.size(); i++) {
                path[i] = new Coords(
                        pathList.get(i).get(0),
                        pathList.get(i).get(1)
                );
            }
            trains.add(new Train(board, id, color, path, coordinatesLock));
        }

        for (Train train : trains) {
            new Thread(train).start();
        }
    }
}
