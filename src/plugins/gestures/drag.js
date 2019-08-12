const POINTER_START_EVENTS = ['mousedown', 'touchstart']
const POINTER_MOVE_EVENTS = ['mousemove', 'touchmove']
const POINTER_END_EVENTS = ['mouseup', 'touchend']

function addEventListeners (el, events, handler) {
  for (var i = 0, len = events.length; i < len; i++) {
    el.addEventListener(events[i], handler, { passive: false })
  }
}

function removeEventListeners (el, events, handler) {
  for (var i = 0, len = events.length; i < len; i++) {
    el.removeEventListener(events[i], handler)
  }
}

function getCoords( e, el ){
  let rect = el.getBoundingClientRect()

  if ( e.changedTouches ){
    e = e.changedTouches[0]
  }

  return {
    clientX: e.clientX
    , clientY: e.clientY
    , layerX: e.clientX - rect.left
    , layerY: e.clientY - rect.top
  }
}

export default {
  inserted(el, binding) {
    var draggedElem
    if (!document){ return }

    function onPointerStart(evt) {
      evt.preventDefault()
      let coords = getCoords( evt, el )
      el.lastCoords = el.firstCoords = {
        x: coords.clientX
        , y: coords.clientY
      }
      binding.value({
        el
        , first: true
        , clientX: coords.clientX
        , clientY: coords.clientY
        , offsetX: 0
        , offsetY: 0
        , layerX: coords.layerX
        , layerY: coords.layerY
        , event: evt
      })
      draggedElem = el
    }

    function onPointerEnd(evt) {
      if (el !== draggedElem) return
      let coords = getCoords( evt, el )
      var offsetX = coords.clientX - el.firstCoords.x
      var offsetY = coords.clientY - el.firstCoords.y
      evt.preventDefault()
      el.lastCoords = null
      binding.value({
        el
        , last: true
        , clientX: coords.clientX
        , clientY: coords.clientY
        , offsetX
        , offsetY
        , layerX: coords.layerX
        , layerY: coords.layerY
        , event: evt
      })
      draggedElem = null
    }

    function onPointerMove(evt) {
      if (el !== draggedElem) return
      evt.preventDefault()
      if (el.lastCoords) {
        var coords = getCoords( evt, el )
        var deltaX = coords.clientX - el.lastCoords.x
        var deltaY = coords.clientY - el.lastCoords.y
        var offsetX = coords.clientX - el.firstCoords.x
        var offsetY = coords.clientY - el.firstCoords.y
        var clientX = coords.clientX
        var clientY = coords.clientY

        binding.value({
          el
          , deltaX
          , deltaY
          , offsetX
          , offsetY
          , clientX
          , clientY
          , layerX: coords.layerX
          , layerY: coords.layerY
          , event: evt
        })
        el.lastCoords = {
          x: coords.clientX
          , y: coords.clientY
        }
      }
    }

    addEventListeners(el, POINTER_START_EVENTS, onPointerStart)
    addEventListeners(
      document.documentElement,
      POINTER_END_EVENTS,
      onPointerEnd
    )
    addEventListeners(
      document.documentElement,
      POINTER_MOVE_EVENTS,
      onPointerMove
    )
  }

  , unbind(el) {
    removeEventListeners(el, POINTER_START_EVENTS)
    removeEventListeners(document.documentElement, POINTER_END_EVENTS)
    removeEventListeners(document.documentElement, POINTER_MOVE_EVENTS)
  }
}
