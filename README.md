# 🎨 Brush 

**TL;DR**: Brush is a domain specific  programming language designed to make generative artwork fun and accessible. It uses the idea of evolution as an abstraction for recursion, and contains various builtin functions for transformations like warping, translating, shrinking/enlarging, and rotating to allows users to easily create generative art pieces.


## Syntax

### Base Shapes
A generative art piece with Brush is a collection of various core shapes, each of which can have multiple generations. 

Each shape has a set of properties that are predefined and can be changed by the user. Some of these are universal (like`center`) and some other ones differ (like `radius`). 

They all have an evolution function which defines where their children will be located. This is meant to abstract away recursion.

#### Universal Properties
- `center` – center of the shape
- `fill` – internal fill of the shape, defaults to transparent
- `outline` – color of the outline, defaults to black
- `width` – width of the given shape, defaults to whatever other parameters are passed 
- `height` – height of the given shape, defaults to whatever other parameters are passed 

All properties can be redefined when a shape is instantiated when drawing it

#### Polygon 
When instantiating a polygon, the only required property is `vertices`
```javascript
/* Creating a variable called triangle that is a polygon */
let triangle = polygon {
    /* All vertices are defined with 0,0 as the midpoint */
    vertices = [(-100, -100), (100, -100), (100, 100)]
    
    /* Red fill */
    fill = rgb(255,0,0)
    
    evolve {
        /* evolve is explained at a later stage */
    }
}
```

#### Circle
When instantiating a circle, the only required property is radius
```javascript
/* Creating a variable called circle1 that is a circle */
let circle1 = circle {
    /* Define the radius of the circle */
    radius = 50
    
    /* Blue fill */
    fill = rgb(0, 0, 255)
    
    evolve {
        /* evolve is explained at a later stage */
    }
}
```

#### Rectangle
Code to instantiate a rectangle (width/height are required parameters)
```javascript
/* Creating a variable called rect1 that is a rectangle */
let rect1 = rectangle {
    /* Define the width and height of the rectangle */
    width = 100
    height = 50
    
    /* Green fill */
    fill = rgb(0, 255, 0)
    
    evolve {
        /* evolve is explained at a later stage */
    }
}

```

#### SVG
Code to load an SVG
```javascript
/* Creating a variable called svg1 that is an SVG import */
let svg1 = svg {
    /* SVG file path */
    file = "file.svg"
    
    /* Define the scale of the SVG */
    scale = 0.5
    
    evolve {
        /* evolve is explained at a later stage */
    }
}

```

### Evolution
Evolution is the core innovation of brush – each base shape can have a custom definition of how they can evolve.

If there is only one child per shape drawn, you can just modify the properties of the base shape:
```javascript
evolve {
    center.y -= radius
    radius *= 2
}
```

If there are multiple children per shape drawn, children are defined using `child { }` with the properties set inside the braces:
```javascript
evolve {
    child {
        radius = radius * 2
        center.y = center.y - radius
    }
    
    child {
        radius = radius * 2
        center.y = center.y + radius
    }
}
```
### Transformation functions
**Shift** – used to translate a given shape, and takes parameters for X and for Y
```javascript
/* This will shift whatever shape by 100 pixels to the right, and 200 pixels up */
shift(100, 200)
```

**Stretch** – used to stretch shapes/SVGs, takes parameters for X and for Y
```javascript
/* This will stretch whatever shape by 1.5x horizontally, and 2x vertically */
stretch(1.5, 2)
```

**Rotate** – used to rotate a given shape around its center, takes an angle parameter in degrees
```javascript
/* This will rotate whatever shape by 45 degrees clockwise */
rotate(45)
```

**Reflect** – used to reflect a given shape across a line defined by two points, takes two point parameters
```javascript
/* This will reflect whatever shape across the line defined by points (0,0) and (1,1) */
reflect((0,0), (1,1))
```

**Warp** – used to warp a given shape using a specified warp function and parameters
```javascript
/* This will warp whatever shape using the "ripple" function with a frequency of 0.5 and amplitude of 0.2 */
warp("ripple", 0.5, 0.2)
```

### Drawing
To draw, you instantiate whatever shapes you've defined above, and can rewrite whatever properties you want to! You can also define the number of generations (how many times the evolve function will be called).
```javascript
circle1(
    /* defines where the first circle is drawn */
    center = (100, 100)
    generations = 8
)
```


Code samples can be found in `examples/`