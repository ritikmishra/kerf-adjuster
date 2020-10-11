import React, {createRef, RefObject, useEffect, useState} from 'react';
import './file-viewing-pane.scss'
import * as THREE from "three";

class ThreeJsObjects {
  constructor(
    public scene: THREE.Scene,
    public camera: THREE.OrthographicCamera,
    public renderer: THREE.Renderer,
    public plane: THREE.Plane,
    public width: number,
    public height: number) {
  }

  render(): void {
    this.renderer.render(this.scene, this.camera);
  }
}

interface Position {
  x: number
  y: number
}

const scaleFactor = 100;

const initThreeJs = (domRef: RefObject<HTMLDivElement>): ThreeJsObjects => {
  // === THREE.JS CODE START ===
  const plane = new THREE.Plane(new THREE.Vector3(0, 0, 1)) // xy-plane
  const scene = new THREE.Scene();
  let height = window.innerHeight;
  let width = window.innerWidth;

  const camera = new THREE.OrthographicCamera(-width / scaleFactor, width / scaleFactor, height / scaleFactor, -height / scaleFactor, 0.1, 1000);
  camera.position.z = 1
  const renderer = new THREE.WebGLRenderer();
  renderer.setSize(width, height);
  if (domRef.current) {
    domRef.current.appendChild(renderer.domElement);
  }

  return new ThreeJsObjects(scene, camera, renderer, plane, width, height);
  // === THREE.JS EXAMPLE CODE END ===
}

const sigmoid = (x: number): number => {
  return 10000 * (Math.atan(-x) + Math.PI / 2) / Math.PI
}

export const FileViewingPane = () => {
  const paneRef = createRef<HTMLDivElement>();
  let [threeJsObjects, setThreeJsObjects] = useState<ThreeJsObjects>();
  let [zoom, setZoom] = useState<number>(1);
  let [pos, setPos] = useState<Position>({x: 0, y: 0})


  // Initialize everything
  useEffect(() => {
    setThreeJsObjects(initThreeJs(paneRef));
  }, [])


  // Add the green box (TODO: delete)
  useEffect(() => {
    if (threeJsObjects) {
      const geometry = new THREE.BoxGeometry(90, 1, 1);
      geometry.translate(6, 3, 0)

      const geometry2 = new THREE.BoxGeometry(1, 1, 1);
      const material = new THREE.MeshBasicMaterial({color: 0x00ff00});
      const cube = new THREE.Mesh(geometry, material);
      threeJsObjects.scene.add(cube, new THREE.Mesh(geometry2, material));
      threeJsObjects.render();
    }
  }, [threeJsObjects])

  // Rerender every time something changes
  useEffect(() => {
    if (threeJsObjects) {
      threeJsObjects.render()
    }
  })

  // Fix scene when window is resized
  useEffect(function handleResize() {
    let width = window.innerWidth;
    let height = window.innerHeight;
    if (threeJsObjects) {
      threeJsObjects.renderer.setSize(width, height)
      threeJsObjects.camera.left = -width / scaleFactor
      threeJsObjects.camera.right = width / scaleFactor
      threeJsObjects.camera.top = height / scaleFactor
      threeJsObjects.camera.bottom = -height / scaleFactor
      threeJsObjects.camera.updateProjectionMatrix()
      threeJsObjects.render()
    }
    window.addEventListener('resize', handleResize)
  })

  // Update display when we zoom
  useEffect(() => {
    if (threeJsObjects) {
      threeJsObjects.camera.zoom = zoom;
      threeJsObjects.camera.updateProjectionMatrix()
    }
  }, [threeJsObjects, zoom])

  // Update camera position when it is changed on drag
  useEffect(() => {
    if (threeJsObjects) {
      threeJsObjects.camera.position.x = pos.x
      threeJsObjects.camera.position.y = pos.y
    }
  }, [threeJsObjects, pos])


  return (
    <div
      className="pane"
      ref={paneRef}
      onWheel={(event) => {
        // TODO: set max zoom based on the size of the object
        setZoom(Math.max(0.001, zoom - (event.deltaY / sigmoid(zoom))));
      }}
      onMouseMove={(event) => {
        if (event.buttons === 1) {
          // TODO: refine the number 50 as a zoom to pixels-per-inch conversion factor
          setPos({
            x: pos.x - event.movementX / (50 * zoom),
            y: pos.y + event.movementY / (50 * zoom)
          })
        }
      }}
    />
  )
}