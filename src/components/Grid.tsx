import {Fragment} from "react";
import {createCoordsKey} from "../displayStateReducer.ts";




const Grid = ({displayState}) => {
    const numOfColumns = 11; // 11
    const numOfRows = 17;

    const squareSide = Math.max(numOfColumns, numOfRows)

    const getCellBgColor = (rowIndex, colIndex) => {
        const cellKey = createCoordsKey(rowIndex, colIndex);
        const cellData = displayState.dynamicPoints.get(cellKey);
        return cellData ? cellData.bg_color : 'white';
    }

    return (
        <div className="grid w-full h-screen" style={{
            gridTemplateColumns: `repeat(${squareSide}, 40px)`,
            gridTemplateRows: `repeat(${squareSide}, 40px)`,
            gap: '10px',
        }}>
            {
                Array.from({ length: squareSide }, (_, rowIndex) => (
                  <Fragment key={rowIndex}>
                      {
                          Array.from({ length: squareSide }, (_, colIndex) => (
                              <div
                                  key={colIndex}
                                  className="border border-gray-400 rounded-xs aspect-square"
                                    style={{
                                        backgroundColor: getCellBgColor(rowIndex, colIndex)
                                    }}
                              >
                                  <span className="opacity-70 font-mono px-[2px] text-xs">({rowIndex}, {colIndex})</span>
                              </div>
                          ))
                      }
                  </Fragment>
                ))
            }
        </div>
    )
}

export default Grid