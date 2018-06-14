import { stringify as QSStringify } from 'qs'

export const REQUEST_SHOWS_LIST = 'REQUEST_SHOWS_LIST'
export const RECEIVE_SHOWS_LIST = 'REQUEST_SHOWS_LIST'
export const REQUEST_SHOW = 'REQUEST_SHOW'
export const RECEIVE_SHOW = 'RECEIVE_SHOW'
export const ADD_SHOW = 'ADD_SHOW'
export const RECEIVE_ADDED_SHOW = 'RECEIVE_ADDED_SHOW'

export function requestShowsList() {
  return {
    type: REQUEST_SHOWS_LIST
  }
}

export function receiveShowsList(json) {
  return {
    type: RECEIVE_SHOWS_LIST,
    shows: json
  }
}

export function requestShow(id) {
  return {
    type: REQUEST_SHOW,
    id
  }
}

export function receiveShow(id, json) {
  return {
    type: RECEIVE_SHOW,
    id,
    show: json
  }
}

export function addShow(provider, providerID) {
  return {
    type: ADD_SHOW,
    provider,
    providerID
  }
}

export function receiveAddedShow(provider, providerID, json) {
  return {
    type: RECEIVE_ADDED_SHOW,
    provider,
    providerID,
    show: json
  }
}

export function fetchShow(id) {
  return dispatch => {
    dispatch(requestShow(id))
    return fetch(`http://localhost:8080/api/shows/${id}`)
      .then(res => res.json())
      .then(json => dispatch(receiveShow(id, json)))
  }
}

export function fetchShowsList() {
  return dispatch => {
    dispatch(requestShowsList())
    return fetch('http://localhost:8080/api/shows')
      .then(res => res.json())
      .then(json => dispatch(receiveShowsList(json)))
  }
}

export function fetchAddShow(provider, providerID) {
  return dispatch => {
    dispatch(addShow(provider, providerID))
    return fetch('http://localhost:8080/api/shows', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded; charset=UTF-8'
      },
      body: QSStringify({
        provider,
        providerID
      })
    })
    .then(res => res.json())
    .then(json => dispatch(receiveAddShow(provider, providerID, json)))
  }
}
