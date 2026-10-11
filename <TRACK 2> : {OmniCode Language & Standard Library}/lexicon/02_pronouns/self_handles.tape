# ==============================================================================
# TAPE HEAD: SELF HANDLES EXECUTION PROCESS RIBBON
# ==============================================================================
# ARCHITECTURAL TOPOLOGY:
#   Executes context pointer resolution for self, caller, and owner.
# ==============================================================================

[Tape.SelfHandles] : WHO {OmniCode_Kernel} WHAT {SelfHandles} --> RESOLVE {ContextMesh}
    WHEN {GenesisBoot}
    WHERE {handle_partition} = [ 64 ]
    WHY ( Test local frame pointers and context handles )
    HOW [ +1 : PROCEED ]
    :: < handle_landmark > ( 1 Corinthians 12:27 - 'Ye are the body of Christ, and members in particular' )

# ==============================================================================
# SUBORDINATE ROLLS: SCOPED CALLER AND FRAME RESOLUTION
# ==============================================================================

# Roll 1: Reference abstract Self, Caller, and Owner specifications
roll_1 [ +Self & +Caller & +Owner ] ( Abstract handle blueprints referenced )

# Roll 2: Assign local frame handle
roll_2 [ self --> Local_Frame_0 @ 0 ] ( Bind active frame to local cell )

# Roll 3: Verify caller resolution invariance
roll_3 [ caller != self ! [ +1 ] ] ( Verify caller and callee frame distinction )

# Roll 4: Commit with affirmative concrete macro
roll_4 [ commit_status [ PROCEED- ] ] ( Self handle pronouns verified and sealed )
