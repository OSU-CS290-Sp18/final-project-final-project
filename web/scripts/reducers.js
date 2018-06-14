import { combineReducers } from 'redux'
import {
  REQUEST_SHOWS_LIST,
  RECEIVE_SHOWS_LIST,
  REQUEST_SHOW,
  RECEIVE_SHOW,
  ADD_SHOW,
  RECEIVE_ADDED_SHOW
} from './actions'

function show(
  state = {
    show: {},
    waiting: false
  },
  action
) {
  switch (action.type) {
    case REQUEST_SHOW:
    case ADD_SHOW:
      return Object.assign({}, state, {
        waiting: true
      })
    case RECEIVE_SHOW:
    case RECEIVE_ADDED_SHOW:
      return Object.assign({}, state, {
        show: state.show,
        waiting: false
      })
    default:
      return state
  }
}

function shows(
  state = {
    shows: [],
    waiting: false
  },
  action
) {
  switch (action.type) {
    case REQUEST_SHOWS_LIST:
      return Object.assign({}, state, {
        waiting: true
      })

    case RECEIVE_SHOWS_LIST:
      return Object.assign({}, state, {
        shows: state.shows,
        waiting: false
      })

    default:
      return state
  }
}

const unifyReducer = combineReducers({
  show,
  shows
})

export default unifyReducer
