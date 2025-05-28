import {produce} from "immer";

export const createCoordsKey = (x: number, y: number) => {
    return `${x}-${y}`;
}
const createPointKey = (point) => {
    const [x, y] = point.coords
    return createCoordsKey(x, y)
}

interface DisplayReducerState {
    dynamicPoints: Map<string, {coords: [number, number], bg_color: string}>
}

interface DisplayActionAddCars {
    display_action_type: 'add_cars'
    display_action_payload: {coords: [number, number], bg_color: string}[]
}

interface DisplayActionClearCoords {
    display_action_type: 'clear_coords'
    display_action_payload: [number, number][]
}

type DisplayAction = DisplayActionAddCars | DisplayActionClearCoords


const displayStateReducer = (state: DisplayReducerState, action: DisplayAction) => {
    const payload = action.display_action_payload
    switch (action.display_action_type) {
        case 'add_cars':
            return produce(state, draft => {
                payload.forEach(point => {
                    draft.dynamicPoints.set(createPointKey(point), point)
                })
            })
        case 'clear_coords':
            return produce(state, draft => {
                payload.forEach(point => {
                    draft.dynamicPoints.delete(createPointKey(point))
                })
            })
        default:
            return state;
    }
}

export default displayStateReducer